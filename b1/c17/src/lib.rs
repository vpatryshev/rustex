
  pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
  }

  impl Post {
    pub fn new() -> Post {
      Post {
        state: Some(Box::new(Draft {})),
        content: String::new(),
      }
    }
    pub fn add_text(&mut self, text: &str) {
      match &self.state {
        None => {}
        Some(s) => {
          if s.canAdd() {
            self.content.push_str(text);
          }
        }
      }
    }
    pub fn content(&self) -> String {
      self.state.as_ref().unwrap().content(&self)
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
    pub fn reject(&mut self) {
      if let Some(s) = self.state.take() {
        self.state = Some(s.reject())
      }
    }
  }

  trait State {
    fn canAdd(self: Box<Self>) -> bool;
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> String {
      "".to_owned() 
    }
  }

  struct Draft {}

  impl State for Draft {
    fn canAdd(self: Box<Self>) -> bool { true }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
      Box::new(PendingReview {count: 0})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
      self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
      self
    }

    fn content<'a>(&self, _post: &'a Post) -> String {
      "this is a draft".to_owned()
    }
  }

  struct PendingReview {
    count: i32,
  }

  impl State for PendingReview {
    fn canAdd(self: Box<Self>) -> bool { self.count < 1 }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
      self
    }
    fn approve(mut self: Box<Self>) -> Box<dyn State> {
      self.count = self.count + 1;
      if self.count < 2 { self }
      else { Box::new(Published {}) }
    }
    
    fn reject(mut self: Box<Self>) -> Box<dyn State> {
      self.count = self.count - 1;
      if self.count < 0 { Box::new(Draft {}) }
      else { self }
    }
    
    fn content<'a>(&'a self, _post: &'a Post) ->  String {
        format!("Need {} approvals", 2 - self.count).to_owned()
    }
  }
  
  struct Published {}

  impl State for Published {
    fn canAdd(self: Box<Self>) -> bool { false }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
      self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
      self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
      self
    }
    
    fn content<'a>(&self, post: &'a Post) -> String {
       post.content.to_owned()
    }
  }