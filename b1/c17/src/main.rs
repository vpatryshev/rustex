use blog::Post;

fn main() {

  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today.");
  assert_eq!("this is a draft", post.content());

  post.request_review();
  assert_eq!("Need 2 approvals", post.content());

  post.reject();
  assert_eq!("this is a draft", post.content());

  post.add_text(" Bad salad.");
  assert_eq!("this is a draft", post.content());

  post.request_review();
  assert_eq!("Need 2 approvals", post.content());

  post.reject();
  assert_eq!("this is a draft", post.content());

  post.approve();
  assert_eq!("this is a draft", post.content());

  post.request_review();
  assert_eq!("Need 2 approvals", post.content());

  post.approve();
  assert_eq!("Need 1 approvals", post.content());

  post.approve();
  assert_eq!("I ate a salad for lunch today. Bad salad.", post.content());

  post.approve();
  assert_eq!("I ate a salad for lunch today. Bad salad.", post.content());

  post.reject();
  assert_eq!("I ate a salad for lunch today. Bad salad.", post.content());
}