use std::cmp::{max, min};
use std::collections::BTreeMap;
use chrono::prelude::*;
use chrono::Duration;
use regex::Regex;

use crate::utils::read_input;

static DATE_TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M";

lazy_static! {
  static ref SHIFT_START_MATCHER: Regex = Regex::new("^\\[(.+)\\] Guard #([0-9]+) begins shift$")
    .unwrap();
  static ref ASLEEP_MATCHER: Regex  = Regex::new("^\\[(.+)\\] falls asleep$")
    .unwrap();
  static ref WAKE_UP_MATCHER: Regex  = Regex::new("^\\[(.+)\\] wakes up$")
    .unwrap();
}

enum GuardState {
  Awake,
  Asleep
}

struct Guard {
  current_state: GuardState,
  current_state_change: NaiveDateTime,
  minutes_asleep: Vec<i32>
}

impl Guard {
  pub fn starts_first_shift(first_shift_start: NaiveDateTime) -> Guard {
    Guard {
      current_state: GuardState::Awake,
      current_state_change: first_shift_start,
      minutes_asleep: vec![0; 60]
    }
  }

  pub fn starts_shift(&mut self, at: NaiveDateTime) {
    if let GuardState::Asleep = self.current_state { self.wakes_up(at) }
  }

  fn report_asleep(&mut self, start_minute: usize, end_minute: usize) {
    let start = max(start_minute, 0);
    let end = min(end_minute, 59);

    for m in &mut self.minutes_asleep[start .. end] {
      *m += 1
    }
  }

  pub fn falls_asleep(&mut self, at: NaiveDateTime) {
    match self.current_state {
      GuardState::Awake => {
        self.current_state = GuardState::Asleep;
        self.current_state_change = at;
      }
      _ => panic!("Guard cannot be more asleep")
    }
  }

  pub fn wakes_up(&mut self, at: NaiveDateTime) {
    match self.current_state {
      GuardState::Asleep => {
        let sleep_dur: Duration = at.signed_duration_since(self.current_state_change);

        self.report_asleep(
          self.current_state_change.minute() as usize,
          (i64::from(self.current_state_change.minute()) + sleep_dur.num_minutes()) as usize
        );

        self.current_state = GuardState::Awake;
        self.current_state_change = at;
      }
      _ => panic!("Guard cannot be more awake")
    }
  }
}

struct GuardsLog {
  current_guard_id: Option<i32>,
  guards: BTreeMap<i32, Guard>
}

impl GuardsLog {
  pub fn new() -> GuardsLog {
    GuardsLog {
      current_guard_id: None,
      guards: BTreeMap::new()
    }
  }
  fn current_guard(&mut self) -> &mut Guard {
    let current_guard_id = self.current_guard_id.unwrap_or_else(|| 
      panic!("Expected guard id to be set"));

    self.guards.get_mut(&current_guard_id)
      .unwrap_or_else(|| panic!("Expected guard {} to be available", current_guard_id))
  }

  fn handle_event(&mut self, line: &ParsedLine) {
    match line {
      ParsedLine::ShiftStart(when, guard_id) => {
        self.current_guard_id = Some(*guard_id);
        let guard: &mut Guard = self.guards.entry(*guard_id).or_insert_with(|| Guard::starts_first_shift(*when));
        guard.starts_shift(*when);
      },
      ParsedLine::FallAsleep(when) => { 
        self.current_guard().falls_asleep(*when);
      },
      ParsedLine::WakeUp(when) => {
        self.current_guard().wakes_up(*when);
      }
    }
  }
}

#[derive(Debug)]
enum ParsedLine {
  ShiftStart(NaiveDateTime, i32),
  FallAsleep(NaiveDateTime),
  WakeUp(NaiveDateTime)
}

fn parse_line(line: &str) -> ParsedLine {
  if SHIFT_START_MATCHER.is_match(line) {
    let captures = SHIFT_START_MATCHER.captures(line)
      .unwrap_or_else(|| panic!("Invalid shift start line line: {}", line));
    let date = NaiveDateTime::parse_from_str(&captures[1], DATE_TIME_FORMAT)
      .unwrap_or_else(|e| panic!("Unable to parse date {:?}: {:?}", &captures[1], e));
    let guard_id = &captures[2].parse::<i32>()
      .unwrap_or_else(|_| panic!("Unable to parse guard id {:?}", &captures[2]));
    
    ParsedLine::ShiftStart(date, *guard_id)
  } else if ASLEEP_MATCHER.is_match(line) {
    let captures = ASLEEP_MATCHER.captures(line)
      .unwrap_or_else(|| panic!("Invalid fall asleep line: {}", line));
    let date = NaiveDateTime::parse_from_str(&captures[1], DATE_TIME_FORMAT)
      .unwrap_or_else(|_| panic!("Unable to parse date {:?}", &captures[1]));
    
    ParsedLine::FallAsleep(date)
  } else if WAKE_UP_MATCHER.is_match(line) {
    let captures = WAKE_UP_MATCHER.captures(line)
      .unwrap_or_else(|| panic!("Invalid wake up line: {}", line));
    let date = NaiveDateTime::parse_from_str(&captures[1], DATE_TIME_FORMAT)
      .unwrap_or_else(|_| panic!("Unable to parse date {:?}", &captures[1]));
    
    ParsedLine::WakeUp(date)
  } else {
    panic!("Line does not match any known type: {}", line);
  }
}

fn read_guards_log(path: &str) -> GuardsLog {
  let mut lines = read_input(path);
  lines.sort_unstable();

  let mut log: GuardsLog = GuardsLog::new();
  lines.iter()
    .map(|l| parse_line(l))
    .for_each(|e| log.handle_event(&e));

  log
}

/// For this one it seems that the easiest solution is to sort the input chronologically
/// and then use a very simple, stateful parser to ingest the events.log
/// 
/// Sorting input seems to be fairly easy, due to a line format it's a simple, alphanumerical
/// sort on lines. According to Rust docs, this should be O(n log n) complexity
/// 
/// After the sort, we need a stateful parser to keep track of who is the current guard and
/// tally-up their sleep minues according to incoming events
/// 
/// Finally, after all the events are ingested, find the "most asleep" guard and
/// find his most asleep minute
/// 
/// It seem stac complexity is
/// O (
///    n log n -> sorting
///    +
///    n -> parsing events
///    +
///    60n -> finding the most asleep guard  
/// )
/// 
/// Probably the last '60n' could be shaven-off if the intermediate representation 'GuardsLog'
/// is ditched and the parser tracks guards to find the most-asleep one. However, this
/// would require separate implementation of specialised parser for part 2
pub fn solve_part1(path: &str) -> i32 {
  let log = read_guards_log(path);

  let (guard_id, guard_data) = log.guards.iter()
    .max_by_key(|i| i.1.minutes_asleep.iter().sum::<i32>())
    .unwrap();

  let (guard_most_asleep_minute, _) = guard_data.minutes_asleep.iter().enumerate()
    .max_by_key(|e| e.1)
    .unwrap();

  guard_id * (guard_most_asleep_minute as i32)
}

/// All but the last steps are the same as in part1
/// 
/// As a final step, just build a triples (guard_id, minute, times_asleep) from parsed log and
/// find the one with the largest value of times_asleep.
/// 
/// Time complexity is same as for part 1.
pub fn solve_part2(path: &str) -> i32 {
  let log = read_guards_log(path);

  let (guard_id, guard_most_asleep_minute, _) = log.guards.iter().flat_map(|(guard_id, guard_data)| {
    guard_data.minutes_asleep.iter().enumerate()
      .map(move |(guard_most_asleep_minute, times_asleep)| (guard_id, guard_most_asleep_minute, times_asleep))
  })
  .max_by_key(|e| e.2)
  .unwrap();

  guard_id * (guard_most_asleep_minute as i32)
}