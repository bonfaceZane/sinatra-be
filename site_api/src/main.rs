use shared::Post;

fn main() {
    let new_post = Post::new("Global warming".to_owned(), "New body".to_owned());

    dbg!(new_post)
}
