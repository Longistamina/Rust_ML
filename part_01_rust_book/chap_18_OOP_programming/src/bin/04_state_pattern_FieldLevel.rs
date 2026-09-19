/*
Another way to design pattern is `field-level state pattern`
=> each struct has a field named `state` to store the state object of that value

-------------------------------------------------------------------

We’re going to work through an example of a blog post struct
that has a field to hold its state, which will be a state object from the set
    + "draft"
    + "review"
    + "published"

Here, the field `state` will implement trait `State`

The final functionality will look like this:
    1. A blog post starts as an empty draft.
    2. When the draft is done, a review of the post is requested.
    3. When the post is approved, it gets published.
    4. Only published blog posts return content to print so that unapproved posts can’t accidentally be published.

Any other changes attempted on a post should have no effect.
For example, if we try to approve a draft blog post before we’ve requested a review,
the post should remain an unpublished draft.
*/

// ==============================================================================
// 1. Defining `Post`, `Draft` and `PendingReview` structs and trait `State`
// ==============================================================================

trait State {
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String
}

struct Draft {}

struct PendingReview {}

struct Published {}

// ======================================================================================================
// 2. Implement methods and State trait for `Post`, `Draft`, `PendingReview` and `Published` structs
// =======================================================================================================

// ---------------- //
// methods for Post //
// ---------------- //

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// ------------------------------- //
// implement trait State for Draft //
// ------------------------------- //

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// --------------------------------------- //
// implement trait State for PendingReview //
// --------------------------------------- //

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// ----------------------------------- //
// implement trait State for Published //
// ----------------------------------- //

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

// ============ //
//    main()    //
// ============ //

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
