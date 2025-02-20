pub mod crop_tool {
    use std::path::{Path, PathBuf};
    use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};
    use image::imageops::FilterType;

    const POSTER_OFFSETS: &[&[u32; 4]; 5] = &[
        &[0, 0, 341, 559],
        &[346, 0, 284, 559],
        &[641, 58, 274, 243],
        &[184, 620, 411, 364],
        &[632, 320, 372, 672],
    ];

    // Templates
    const POSTER_TEMPLATE: &str = "posters_template.png";
    const PAINTING_TEMPLATE: &str = "painting_template.png";

    // Output paths
    const PLUGIN_PATH: &str = "BepInEx/plugins";
    const POSTERS_PATH: &str = "LethalPosters/posters";
    const TIPS_PATH: &str = "LethalPosters/tips";
    const PAINTINGS_PATH: &str = "LethalPaintings/paintings";

    pub struct CropParams {
        pub input: String,
        pub output: String,
        pub template: String,
        // modes: Vec<String>,
    }

    pub fn generate(params: &CropParams) {
        println!("Generating Crop Tool");

        let painting_template = get_template(&params.template, PAINTING_TEMPLATE);
        let poster_template = get_template(&params.template, POSTER_TEMPLATE);
        let poster_dir = get_output_path(&params.output, POSTERS_PATH);
        let tips_dir = get_output_path(&params.output, TIPS_PATH);
        let paintings_dir = get_output_path(&params.output, PAINTINGS_PATH);


        let p = read_input_pictures(&params.input);

        for i in 0..p.len() {
            let tag = format!("{i}.png");

            generate_atlas(
                &poster_template,
                &[
                    g(&p, i),
                    g(&p, i + 1),
                    g(&p, i + 2),
                    g(&p, i + 3),
                    g(&p, i + 4),
                ],
            )
                .save_with_format(poster_dir.join(&tag).as_path(), ImageFormat::Png)
                .unwrap();

            generate_tips(g(&p, i))
                .save_with_format(tips_dir.join(&tag).as_path(), ImageFormat::Png)
                .unwrap();

            generate_painting(&painting_template, g(&p, i))
                .save_with_format(paintings_dir.join(&tag).as_path(), ImageFormat::Png)
                .unwrap();

            println!("{:?}", poster_dir.join(&tag).as_path());
            println!("{:?}", tips_dir.join(&tag).as_path());
            println!("{:?}", paintings_dir.join(&tag).as_path());
        }

        println!("Generation complete !");
    }

    fn get_template(uri: &str, template: &str) -> DynamicImage {
        let path = Path::new(uri).join(template);
        println!("Reading template {}", path.display());
        image::open(path).unwrap()
    }

    fn get_output_path(uri: &str, dir: &str) -> PathBuf {
        let path = PathBuf::from(uri).join(PLUGIN_PATH).join(dir);
        std::fs::create_dir_all(&path).unwrap();
        println!("Reading output {}", path.display());
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

    fn generate_atlas(template: &DynamicImage, posters: &[&DynamicImage; 5]) -> DynamicImage {
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

    fn g<'a>(input: &'a Vec<DynamicImage>, index: usize) -> &'a DynamicImage {
        input.get(index % input.len()).unwrap()
    }
}