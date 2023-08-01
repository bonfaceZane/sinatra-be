use image_api::Image;
use shared::Post;

fn main() {
    let new_post = Post::new("Global warming".to_string(), "New body".to_string());

    let second_post = Post::new(
        "Best losless music".to_owned(),
        "Create.Share.Make".to_owned(),
    );

    let get_new_image = Image::new("perfomant image".to_owned());

    dbg!(new_post);
    dbg!(second_post);
    dbg!(get_new_image);
}
