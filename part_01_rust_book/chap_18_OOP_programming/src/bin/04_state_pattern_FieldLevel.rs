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

//-------------//
// State trait //
//-------------//

trait State {
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;
}

//---------------//
// other structs //
//---------------//

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
        self.state.as_ref().unwrap().content(self) // use `as_ref()` to get the reference without taking the ownership of the `self.state`
        /*
        We don't return the `self.content` here, instead we access the content from `self.state`

        At `Draft` and `PendingReview` states,
        calling `self.content()` will return the empty string ""
        as defined by the default `fn content()` in `State` trait.

        Only at `Published` state, the method `fn content()` is rewritten
        to return the actual `self.content` string
        */
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
        /*
        Why `self.state.take()`?

        Remind, `self.state` is `Option<T>`.
        So if we just use `if let Some(s) = self.state {}`,
        it will move the `Box<dyn State>` inside `self.state` to the `s`,
        then the `self.state` is consumed and dropped,
        => making the next line `self.state = Some(s.request_review())` impossible

        Therefore, we need to use `self.state.take()`,
        it will move the `Box<dyn State>` inside `self.state` to the `s`,
        but then put `None` at the place of `self.state`,
        so the `self.state` is still valid and is not dropped
        => the next line `self.state = Some(s.request_review())` is now possible
        */
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

    fn content<'a>(&self, post: &'a Post) -> &'a str { // redefine `content()` method here to return the actual content at `Published` state
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

/*
+---------------------+
|   Post::new()       |
+---------------------+
        |
        v
+---------------------+
| state: Some(Draft)  |
| content: ""         |
| self.content(): ""  |  <- Draft returns empty string
+---------------------+
        |
        v  (add_text)
+---------------------+
| state: Some(Draft)  |
| content: "I ate..." |
| self.content(): ""  |  <- Still Draft, returns empty
+---------------------+
        |
        v  (request_review)
+----------------------------+
| state: Some(PendingReview) |
| content: "I ate..."        |
| self.content(): ""         |  <- PendingReview returns empty
+----------------------------+
        |
        v  (approve)
+------------------------------------+
| state: Some(Published)             |
| content: "I ate..."                |
| self.content(): "I ate a salad..." |  <- Published returns actual content (based on the specified `content()` method of `Published`)
+------------------------------------+
        |
        v  (content())
+---------------------+
| Returns "I ate..."  |
+---------------------+

--------------------------------------------------------------------

Key field changes:
1. Initial: state=Some(Draft), content=""
2. After add_text: state=Some(Draft), content="I ate...", self.content()=""
3. After request_review: state=Some(PendingReview), content="I ate...", self.content()=""
4. After approve: state=Some(Published), content="I ate...", self.content()="I ate..."
5. content() call: Returns the string from Published state

The state field transitions through Box<dyn State> objects:
• Draft → PendingReview (via request_review)
• PendingReview → Published (via approve)
• Published remains Published for any further state changes
The content field remains unchanged throughout all transitions.
*/
