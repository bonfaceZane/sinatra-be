use shared::Post;

fn main() {
    let new_post = Post::new("Global warming".to_owned(), "New body".to_owned());

    let second_post = Post::new(
        "Best losless music".to_owned(),
        "Create.Share.Make".to_owned(),
    );

    dbg!(new_post);
    dbg!(second_post);
}
