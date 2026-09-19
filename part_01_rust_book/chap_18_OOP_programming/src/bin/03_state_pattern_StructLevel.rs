/*
The `state pattern` is an object-oriented design pattern.

The crux of the pattern is that
we define a set of states a value can have internally.
The value’s behavior changes based on its state.

The states are represented by a set of state objects.
Each state object is responsible for its own behavior
and for governing when it should change into another state.
(the value knows nothing about the different behavior of the states
or when to transition between states.)

In Rust, of course, we use structs and traits
rather than objects and inheritance
to create state objects.

The advantage of using the state pattern is that,
when the business requirements of the program change,
we won't need to change the code of the value.
We’ll only need to update the code inside one of the state objects
to change its rules or perhaps add more state objects.

------------------------------------------------------------------------

One way to design a state pattern is `struct-level state pattern`
=> different structs represent different states.

------------------------------------------------------------------------

We’re going to work through an example of a blog post
to demonstrate struct-level state pattern.

We will create different structs to represent different states of the post:
    + “post” ~ published
    + “draft”
    + “review”

The final functionality will look like this:
    1. A blog post starts as an empty draft.
    2. When the draft is done, a review of the post is requested.
    3. When the post is approved, it gets published.
    4. Only published blog posts return content to print so that unapproved posts can’t accidentally be published.

Any other changes attempted on a post should have no effect.
For example, if we try to approve a draft blog post before we’ve requested a review,
the post should remain an unpublished draft.
*/

// ===========================================================
// 1. Create `Post`, `Draft` and `PendingReview` structs
// ===========================================================

pub struct Post {
    content: String,
}

pub struct Draft {
    content: String,
}

pub struct PendingReview {
    content: String,
}

/*
All `Post`, `Draft` and `Pending` have private `content` field,
the only way to access the `content` is via the `Post.content()` method below,
which means that only the content of `Post` can be accessed,
and only be able to be displayed after drafting and reviewing
*/

// ===========================================================
// 2. Define methods for `Post`, `Draft` and `PendingReview`
// ===========================================================

// ---------------- //
// methods for Post //
// ---------------- //

impl Post {
    pub fn new() -> Draft { // when we call `Post::new()`, it will returns a `Draft` instance containing an empty String
        Draft {content: String::new()}
    }

    pub fn content(&self) -> &str { // call `post.content()` returns `&post.content`
        &self.content
    }
}

// ----------------- //
// methods for Draft //
// ----------------- //

impl Draft {
    pub fn add_text(&mut self, text: &str) { // allows add new text to the empty `Draft {empty}` returned by `Post::new()`
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReview {
        PendingReview {content: self.content}
        // when calling `draft.request_review()`, moves the ownership of `draft.content` to `PendingReview`
        // => the `Draft` instance is consumed, no more valid
    }
}

// ------------------------ //
// method for PendingReview //
// ------------------------ //

impl PendingReview {
    pub fn approve(self) -> Post {
        Post {content: self.content}
    }
}
// Again, when calling `review.approve()`, it moves the ownership of `review.content` to `Post {}`.
// Now, the `PendingReview` instance is consumed, and the `Post.content` does not hold empty String anymore
// => `post.content()` will display the content added by `draft.add_text()`.


// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    let mut post = Post::new(); // return a `Draft {String::new()}` instance

    post.add_text("Will AI take over all of us?");

    // println!("{}", post.content());
    // Cannot print `post.content()` here yet, because `Draft` instance does not have `content` method

    let post = post.request_review(); // as said `draft.request_review()` will consume draft, so we need to assign to get the `PendingReview`
    let post = post.approve(); // consumes `PendingReview`, returns `Post` instance with added text from `add_text()`

    println!("{}", post.content());
}
