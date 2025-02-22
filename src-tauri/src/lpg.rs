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

    #[derive(Eq, PartialEq)]
    pub enum Modes {
        Posters,
        Paintings,
    }

    pub struct CropParams {
        pub input: String,
        pub output_dir: PathBuf,
        pub template_dir: PathBuf,
        pub modes: Vec<Modes>,
    }

    pub async fn generate(params: &CropParams) {
        // Pictures to transform
        let original_pictures = read_input_pictures(&params.input);

        // Parallel generation
        let mut tasks = Vec::new();

        if params.modes.contains(&Modes::Posters) {
            let task = generate_posters_task(
                &params.template_dir,
                &params.output_dir,
                original_pictures.clone(),
            );
            tasks.push(task);
        }

        if params.modes.contains(&Modes::Paintings) {
            let task = generate_paintings_task(
                &params.template_dir,
                &params.output_dir,
                original_pictures.clone(),
            );
            tasks.push(task);
        }

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

    fn read_input_pictures(uri: &str) -> Vec<DynamicImage> {
        std::fs::create_dir_all(uri).unwrap();
        std::fs::read_dir(uri)
            .unwrap()
            .flat_map(Result::ok)
            .map(|f| f.path())
            .map(image::open)
            .flat_map(Result::ok)
            .collect()
    }

    /*  Tasks   */
    fn generate_posters_task(
        template: &PathBuf,
        output: &PathBuf,
        pictures: Vec<DynamicImage>,
    ) -> JoinHandle<()> {
        let poster_template = get_template(template, POSTER_TEMPLATE);
        let poster_dir = get_output_path(output, POSTERS_PATH);
        let tips_dir = get_output_path(output, TIPS_PATH);

        async_runtime::spawn_blocking(move || {
            for i in 0..pictures.len() {
                let tag = format!("{i}.png");

                let posters: Vec<&DynamicImage> = (0..5).map(|j| g(&pictures, i + j)).collect();

                generate_atlas(&poster_template, &posters)
                    .save_with_format(poster_dir.join(&tag).as_path(), ImageFormat::Png)
                    .unwrap();

                generate_tips(g(&pictures, i))
                    .save_with_format(tips_dir.join(&tag).as_path(), ImageFormat::Png)
                    .unwrap();
            }
        })
    }

    fn generate_paintings_task(
        template: &PathBuf,
        output: &PathBuf,
        pictures: Vec<DynamicImage>,
    ) -> JoinHandle<()> {
        let painting_template = get_template(template, PAINTING_TEMPLATE);
        let paintings_dir = get_output_path(output, PAINTINGS_PATH);

        async_runtime::spawn_blocking(move || {
            for i in 0..pictures.len() {
                let tag = format!("{i}.png");

                generate_painting(&painting_template, g(&pictures, i))
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

    fn g(input: &Vec<DynamicImage>, index: usize) -> &DynamicImage {
        input.get(index % input.len()).unwrap()
    }
}

pub mod package_tool {
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::PathBuf;
    use walkdir::WalkDir;
    use zip::write::SimpleFileOptions;

    pub async fn read_metadata(uri: PathBuf, package_name: &str) {
        // TODO implement
    }

    pub async fn create(from_dir: PathBuf, uri: PathBuf, package_name: &str) {
        std::fs::create_dir_all(uri.clone()).unwrap();
        let package = format!("{package_name}.zip");
        let to_path = uri.join(package);
        let to_file = File::create(to_path.clone()).unwrap();
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

            let mut file = File::open(path).unwrap();
            let mut buffer = Vec::new();

            file.read_to_end(&mut buffer).unwrap();
            zip.write_all(&buffer).unwrap();
        }

        zip.finish().unwrap();
    }

    pub async fn load(uri: PathBuf, package_name: &str) {
        // TODO implement
    }
}
