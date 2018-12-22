use std::iter::repeat_with;
use std::cell::RefCell;
use std::rc::Rc;

pub fn solve_part1(players: usize, last_marble: u32) -> u32 {
  let mut next_id = 1_u32;
  let marbles = repeat_with(|| { 
    let current = next_id;
    next_id += 1;
    current
  });

  let mut circle: Vec<u32> = vec![0];
  let mut current_marble_position = 0_usize;
  let mut players = vec!(0u32; players);

  for (round, new_marble) in marbles.enumerate() {
    let current_player = round % players.len();
    if new_marble % 23 == 0 {
      let removed_marble_position = if current_marble_position < 7 {
        circle.len() + current_marble_position - 7
      } else {
        current_marble_position - 7
      };
      players[current_player] += new_marble + circle[removed_marble_position];
      circle.remove(removed_marble_position);
      current_marble_position = if removed_marble_position >= circle.len() {
        0
      } else { removed_marble_position };
    } else {
      let mut next_marble_position = (current_marble_position + 2) % circle.len();
      if next_marble_position == 0 {
        next_marble_position = circle.len();
      }
      circle.insert(next_marble_position, new_marble);
      current_marble_position = next_marble_position;
    }
    
    if new_marble == last_marble {
      break;
    }
  }

  *players.iter().max().unwrap()
}

#[derive(PartialEq, Eq)]
struct Marble {
  value: u32,
  next: Rc<RefCell<Marble>>,
  prev: Rc<RefCell<Marble>>,
}

impl Marble {
  fn new_circle() -> Rc<RefCell<Marble>> {
    let first = Rc::new(RefCell::new(Marble {
      value: 0,
      next: unsafe { std::mem::zeroed() },
      prev: unsafe { std::mem::zeroed() }
    }));

    std::mem::forget(
      std::mem::replace(&mut first.borrow_mut().next, first.clone())
    );
    std::mem::forget(
      std::mem::replace(&mut first.borrow_mut().prev, first.clone())
    );
    
    first
  }

  fn move_cw(&self, n: usize) -> Rc<RefCell<Marble>> {
    if n == 0 {
      self.next.borrow().move_ccw(1)
    } else if n == 1 {
      self.next.clone()
    } else {
      self.next.borrow().move_cw(n - 1)
    }
  }

  fn move_ccw(&self, n: usize) -> Rc<RefCell<Marble>> {
    if n == 0 {
      self.prev.borrow().move_cw(1)
    } else if n == 1 {
      self.prev.clone()
    } else {
      self.prev.borrow().move_ccw(n - 1)
    }
  }

  fn append_after(&mut self, marble_value: u32) -> Rc<RefCell<Marble>> {
    let new_prev_ref = { 
      self.next.try_borrow()
        .map(|n| n.prev.clone())
        .ok()
        .unwrap_or_else(|| self.prev.clone())
    };
    let new_next_ref = self.next.clone();

    let new_marble = Rc::new(RefCell::new(Marble {
      value: marble_value,
      next: new_next_ref,
      prev: new_prev_ref
    }));
    
    {
      let mut next_b = self.next.try_borrow_mut();
      match next_b.as_mut() {
        Ok(n) => { n.prev = new_marble.clone() },
        Err(_) => { self.prev = new_marble.clone() }
      };
    }

    self.next = new_marble.clone();
    new_marble
  }

  fn remove(&mut self) -> Rc<RefCell<Marble>> {
    { self.prev.borrow_mut().next = self.next.clone(); }
    { self.next.borrow_mut().prev = self.prev.clone(); }

    self.next.clone()
  }

  fn _circle(&self) -> Vec<u32> {
    let mut result = vec![self.value];
    
    let mut node = self.next.clone();
    loop {
      node = {
        let c = node.borrow();
        if c.value == self.value {
          break;
        }
        result.push(c.value);
        c.next.clone()
      }
    }

    result
  }

  fn is_single(&self) -> bool {
    self.value == { self.next.borrow().value }
  }

  fn _circle_rev(&self) -> Vec<u32> {
    let mut result = vec![self.value];
    
    let mut node = self.prev.clone();
    loop {
      node = {
        let c = node.borrow();
        if c.value == self.value {
          break;
        }
        result.push(c.value);
        c.prev.clone()
      }
    }

    result
  }
}

/// Wow... this was quite a lesson.
/// 
/// Definetly shows how some of the borrow semantics in Rust make
/// some things really, really difficult.
pub fn solve_part2(players: usize, last_marble: u32) -> u32 {
  let mut next_id = 1_u32;
  let marbles = repeat_with(|| { 
    let current = next_id;
    next_id += 1;
    current
  });

  let mut current_marble = Marble::new_circle();
  let mut players = vec!(0u32; players);


  for (round, new_marble) in marbles.enumerate() {
    let current_player = round % players.len();
    
    if new_marble % 23 == 0 {
      current_marble = {
        let to_rem = { current_marble.borrow().move_ccw(7) };
        let mut r = to_rem.borrow_mut();
        players[current_player] += new_marble + r.value;
        r.remove()
      };
    } else if current_marble.borrow().is_single() {
      current_marble = {
        let mut c = current_marble.borrow_mut();
        c.append_after(new_marble)
      };
    } else {
      current_marble = {
        let next = {
          let c = current_marble.borrow();
          c.move_cw(1)
        };
        let mut n = next.borrow_mut();
        n.append_after(new_marble)
      };
    }
    
    if new_marble == last_marble {
      break;
    }
  }

  *players.iter().max().unwrap()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_solution_part1() {
    assert_eq!(solve_part1(9, 25), 32);
    assert_eq!(solve_part1(10, 1618), 8317);
    assert_eq!(solve_part1(13, 7999), 146373);
    assert_eq!(solve_part1(17, 1104), 2764);
    assert_eq!(solve_part1(21, 6111), 54718);
    assert_eq!(solve_part1(30, 5807), 37305);
  }

  #[test]
  fn test_solution_part2() {
    assert_eq!(solve_part2(9, 25), 32);
    assert_eq!(solve_part2(10, 1618), 8317);
    assert_eq!(solve_part2(13, 7999), 146373);
    assert_eq!(solve_part2(17, 1104), 2764);
    assert_eq!(solve_part2(21, 6111), 54718);
    assert_eq!(solve_part2(30, 5807), 37305);
  }

}