pub mod crop_tool {
    use image::imageops::FilterType;
    use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};
    use std::path::PathBuf;
    use tauri::async_runtime;
    use tauri::async_runtime::JoinHandle;

    const POSTER_OFFSETS: &[&[u32; 4]; 5] = &[
        &[0, 0, 341, 559],
        &[346, 0, 284, 559],
        &[641, 58, 274, 243],
        &[184, 620, 411, 364],
        &[632, 320, 372, 672],
    ];

    // Templates
    pub const TEMPLATE_DIR: &str = "templates";
    const POSTER_TEMPLATE: &str = "posters_template.png";
    const PAINTING_TEMPLATE: &str = "painting_template.png";

    // Output paths
    const POSTERS_PATH: &str = "LethalPosters\\posters";
    const TIPS_PATH: &str = "LethalPosters\\tips";
    const PAINTINGS_PATH: &str = "LethalPaintings\\paintings";

    #[derive(serde::Deserialize, Clone)]
    pub struct ListedFile {
        pub path: String,
        pub poster: bool,
        pub painting: bool,
    }

    #[derive(Clone)]
    pub struct OpenedListedFile {
        pub image: DynamicImage,
        pub poster: bool,
        pub painting: bool,
    }
    pub async fn generate(listed_files: Vec<ListedFile>, output_dir: PathBuf, template_dir: PathBuf) {
        // Pictures to transform
        let original_pictures = read_input_pictures(&listed_files);

        // Parallel generation
        let mut tasks = Vec::new();

        let task = generate_posters_task(
            &template_dir,
            &output_dir,
            original_pictures.clone(),
        );
        tasks.push(task);

        let task = generate_paintings_task(
            &template_dir,
            &output_dir,
            original_pictures.clone(),
        );
        tasks.push(task);

        futures::future::join_all(tasks).await;
    }

    /*  Getters */
    fn get_template(uri: &PathBuf, template: &str) -> DynamicImage {
        let path = PathBuf::from(uri).join(template);
        image::open(path).unwrap()
    }

    fn get_output_path(uri: &PathBuf, dir: &str) -> PathBuf {
        let path = PathBuf::from(uri).join(dir);
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    fn read_input_pictures(files: &Vec<ListedFile>) -> Vec<OpenedListedFile> {
        files
            .iter()
            .map(|lf| OpenedListedFile {
                image: image::open(&lf.path).unwrap(),
                poster: lf.poster,
                painting: lf.painting,
            })
            .collect()
    }

    /*  Tasks   */
    fn generate_posters_task(
        template: &PathBuf,
        output: &PathBuf,
        selected_files: Vec<OpenedListedFile>,
    ) -> JoinHandle<()> {
        let poster_template = get_template(template, POSTER_TEMPLATE);
        let poster_dir = get_output_path(output, POSTERS_PATH);
        let tips_dir = get_output_path(output, TIPS_PATH);

        async_runtime::spawn_blocking(move || {
            for i in 0..selected_files.len() {
                if !selected_files.get(i).unwrap().poster {
                    continue;
                }

                let tag = format!("{i}.png");

                let posters: Vec<&DynamicImage> = (0..5).map(|j| g(&selected_files, i + j)).collect();

                generate_atlas(&poster_template, &posters)
                    .save_with_format(poster_dir.join(&tag).as_path(), ImageFormat::Png)
                    .unwrap();

                generate_tips(g(&selected_files, i))
                    .save_with_format(tips_dir.join(&tag).as_path(), ImageFormat::Png)
                    .unwrap();
            }
        })
    }

    fn generate_paintings_task(
        template: &PathBuf,
        output: &PathBuf,
        selected_files: Vec<OpenedListedFile>,
    ) -> JoinHandle<()> {
        let painting_template = get_template(template, PAINTING_TEMPLATE);
        let paintings_dir = get_output_path(output, PAINTINGS_PATH);

        async_runtime::spawn_blocking(move || {
            for i in 0..selected_files.len() {
                if !selected_files.get(i).unwrap().painting {
                    continue;
                }

                let tag = format!("{i}.png");

                generate_painting(&painting_template, g(&selected_files, i))
                    .save_with_format(paintings_dir.join(&tag).as_path(), ImageFormat::Png)
                    .unwrap();
            }
        })
    }

    /*  Generators  */
    fn generate_atlas(template: &DynamicImage, posters: &Vec<&DynamicImage>) -> DynamicImage {
        let mut base = template.clone();
        for (i, o) in POSTER_OFFSETS.iter().enumerate() {
            let p = posters[i].resize(o[2], o[3], FilterType::Lanczos3);
            image::imageops::overlay(&mut base, &p, (o[0] + o[2] - p.width()) as i64, o[1] as i64);
        }
        base
    }

    fn generate_tips(poster: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut base = ImageBuffer::new(796, 1024);
        let p = poster.resize(796, 1024, FilterType::Lanczos3);
        image::imageops::overlay(&mut base, &p, (796 - p.width()) as i64, 0);
        base
    }

    fn generate_painting(template: &DynamicImage, poster: &DynamicImage) -> DynamicImage {
        let mut base = template.clone();
        let p = poster.resize_to_fill(243, 324, FilterType::Lanczos3);
        image::imageops::overlay(&mut base, &p, 264, 19);
        base
    }

    fn g(input: &Vec<OpenedListedFile>, index: usize) -> &DynamicImage {
        &input.get(index % input.len()).unwrap().image
    }
}

pub mod package_tool {
    use std::fs;
    use std::io;
    use std::io::{Read, Write};
    use std::path::PathBuf;
    use walkdir::WalkDir;
    use zip::read::ZipFile;
    use zip::write::SimpleFileOptions;
    use zip::ZipArchive;

    // Output paths
    const POSTERS_PATH: &str = "LethalPosters";
    const PAINTINGS_PATH: &str = "LethalPaintings";

    pub async fn create(from_dir: PathBuf, uri: PathBuf, package_name: &str) {
        fs::create_dir_all(uri.clone()).unwrap();
        let package = format!("{package_name}.zip");
        let to_path = uri.join(package);
        let to_file = fs::File::create(to_path.clone()).unwrap();
        let mut zip = zip::ZipWriter::new(to_file);

        let options = SimpleFileOptions::default();

        let iter = WalkDir::new(&from_dir).into_iter().map(|res| res.unwrap());
        for entry in iter {
            let path = entry.path();
            if path.is_dir() {
                continue; // skip dir
            }

            let relative_path = path.strip_prefix(&from_dir).unwrap().to_owned();
            zip.start_file(relative_path.to_string_lossy().replace("\\", "/"), options)
                .unwrap();

            let mut file = fs::File::open(path).unwrap();
            let mut buffer = Vec::new();

            file.read_to_end(&mut buffer).unwrap();
            zip.write_all(&buffer).unwrap();
        }

        zip.finish().unwrap();
    }

    pub async fn load(uri: PathBuf, package_name: &str, to_dir: PathBuf) {
        let package = format!("{package_name}.zip");
        let from_path = uri.join(package);
        println!("Loading from {}", from_path.display());
        let file = fs::File::open(&from_path).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();

        // TODO validate to_dir exists

        // TODO validate that package is valid
        // User should not be able to load anything else in the output

        // Clean output
        clean_dir(&to_dir);

        // Decompress
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let out_path = to_dir.join(file.mangled_name());

            unzip(&out_path, &mut file);
        }
    }

    fn clean_dir(path: &PathBuf) {
        let poster_path = path.join(POSTERS_PATH);
        if poster_path.exists() {
            println!("Cleaning > {}", poster_path.display());
            fs::remove_dir_all(&poster_path).unwrap();
        }
        let painting_path = path.join(PAINTINGS_PATH);
        if painting_path.exists() {
            println!("Cleaning > {}", painting_path.display());
            fs::remove_dir_all(&painting_path).unwrap();
        }
    }

    fn unzip(out_path: &PathBuf, file: &mut ZipFile) {
        // Ensure parent directories exist
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        // Extract file
        let mut outfile = fs::File::create(&out_path).unwrap();
        io::copy(file, &mut outfile).unwrap();
    }
}

pub mod settings {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Settings {
        pub output: String,
    }
}