use rust_book::Post;

fn main() {
    let mut post = Post::new();

    // draft
    post.add_text("I ate a salad");
    assert_eq!("", post.content());

    // review
    post.request_review();
    post.add_text(" for lunch today");
    assert_eq!("", post.content());
    post.approve(); // first reviewer approves
    post.reject(); // second reviewer rejects, back to draft

    // draft 2.0 implement changes
    post.add_text(" for dinner today");

    // review
    post.request_review();
    post.approve();
    post.approve();

    // published
    assert_eq!("I ate a salad for dinner today", post.content());
}
