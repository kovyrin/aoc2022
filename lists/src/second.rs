pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem: elem,
            next: self.head.take()
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
      self.head.take().map( |node| {
        self.head = node.next;
        node.elem
      })
    }

    pub fn peek(&mut self) -> Option<&T> {
      self.head.as_ref().map(|node| {
        &node.elem
      })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
      self.head.as_mut().map(|node| {
        &mut node.elem
      })
    }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
      let mut cur_link = self.head.take();
      while let Some(mut boxed_node) = cur_link {
          cur_link = boxed_node.next.take();
      }
  }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn int() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        assert_eq!(list.pop(), Some(1));

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn str() {
      let mut list = List::new();

      assert_eq!(list.pop(), None);
      list.push("hello");
      assert_eq!(list.pop(), Some("hello"));

      list.push("one");
      list.push("two");
      list.push("three");

      assert_eq!(list.pop(), Some("three"));
      assert_eq!(list.pop(), Some("two"));

      list.push("four");
      assert_eq!(list.pop(), Some("four"));

      assert_eq!(list.pop(), Some("one"));
      assert_eq!(list.pop(), None);
  }

  #[test]
  fn peek() {
    let mut list = List::new();

    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);

    list.push("hello");
    list.push("banana");

    assert_eq!(list.peek(), Some(&"banana"));
    assert_eq!(list.peek_mut(), Some(&mut "banana"));

    list.peek_mut().map( |value| {
      *value = "yo";
    });

    assert_eq!(list.peek(), Some(&"yo"));
    assert_eq!(list.peek_mut(), Some(&mut "yo"));
  }
}
