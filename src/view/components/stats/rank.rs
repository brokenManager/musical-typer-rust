pub fn rank(acc: f64) -> &'static str {
  if 200.0 <= acc {
    "Wow"
  } else if 150.0 <= acc {
    "Unexpected"
  } else if 125.0 <= acc {
    "Very God"
  } else if 100.0 <= acc {
    "God"
  } else if 99.5 <= acc {
    "Pro"
  } else if 99.0 <= acc {
    "Genius"
  } else if 98.0 <= acc {
    "Geki-tsuyo"
  } else if 97.0 <= acc {
    "tsuyotsuyo"
  } else if 94.0 <= acc {
    "AAA"
  } else if 90.0 <= acc {
    "AA"
  } else if 80.0 <= acc {
    "A"
  } else if 60.0 <= acc {
    "B"
  } else if 40.0 <= acc {
    "C"
  } else if 20.0 <= acc {
    "D"
  } else if 10.0 <= acc {
    "E"
  } else {
    "F"
  }
}

#[test]
fn test_rank() {
  assert_eq!("Wow", rank(256.0));
  assert_eq!("Unexpected", rank(199.9));
  assert_eq!("Very God", rank(149.9));
  assert_eq!("God", rank(124.9));
  assert_eq!("Pro", rank(99.9));
  assert_eq!("Genius", rank(99.49));
  assert_eq!("Geki-tsuyo", rank(98.9));
  assert_eq!("tsuyotsuyo", rank(97.9));
  assert_eq!("AAA", rank(96.9));
  assert_eq!("AA", rank(93.9));
  assert_eq!("A", rank(89.9));
  assert_eq!("B", rank(79.9));
  assert_eq!("C", rank(59.9));
  assert_eq!("D", rank(39.9));
  assert_eq!("E", rank(19.9));
  assert_eq!("F", rank(0.0));
}
