use yangon::*;
#[test]
fn test_all_valid_unicode_planes() {
    let mut y = Yangon::<65536>::with_capacity();
    let test_chars = vec![
        '\u{0041}',      
        '\u{00E9}',      
        '\u{0416}',      
        '\u{0939}',      
        '\u{3042}',      
        '\u{4E00}',      
        '\u{AC00}',      
        '\u{1F600}',     
        '\u{1F680}',     
        '\u{20000}',     
        '\u{2A6D6}',     
        '\u{E0001}',    
    ];
    for ch in test_chars {
        y.push(ch).unwrap();
    }
    let chars: Vec<char> = y.as_str().chars().collect();
    assert!(chars.len() >= 10);
}
#[test]
fn test_combining_diacritical_marks() {
    let mut y = Yangon::<1024>::with_capacity();
    y.push('e').unwrap();
    y.push('\u{0301}').unwrap();
    y.push('\u{0308}').unwrap();
    y.push('\u{0304}').unwrap(); 
    let char_count = y.as_str().chars().count();
    assert_eq!(char_count, 4);
    y.pop();
    y.pop();
    assert_eq!(y.as_str().chars().count(), 2);
}
#[test]
fn test_zero_width_characters() {
    let mut y = Yangon::<1024>::with_capacity();
    y.push('a').unwrap();
    y.push('\u{200B}').unwrap(); 
    y.push('b').unwrap();
    y.push('\u{200C}').unwrap(); 
    y.push('c').unwrap();
    y.push('\u{FEFF}').unwrap(); 
    assert_eq!(y.as_str().chars().count(), 6);
    assert_eq!(y.len(), 12); 
}
#[test]
fn test_right_to_left_and_bidirectional() {
    let mut y = Yangon::<1024>::with_capacity();
    y.push_str("Hello").unwrap();
    y.push_str("مرحبا").unwrap(); 
    y.push_str("שלום").unwrap();  
    y.push_str("World").unwrap();
    y.pop();
    y.insert(0, '!');
    y.remove(1);
    assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
}
#[test]
fn test_emoji_sequences_and_modifiers() {
    let mut y = Yangon::<2048>::with_capacity();
    y.push('👋').unwrap();
    y.push_str("👋🏽").unwrap();
    y.push_str("👨‍👩‍👧‍👦").unwrap();
    y.push_str("🇺🇸").unwrap();
    let byte_len = y.len();
    assert!(byte_len > 20); 
}
#[test]
fn test_malformed_utf8_boundaries() {
    let test_cases = vec![
        "a🦀b",
        "🦀🦀🦀",
        "世界こんにちは",
        "test\u{0301}\u{0308}ing",
    ];
    for test_str in test_cases {
        let mut y = Yangon::<1024>::from(test_str);
        for _ in 0..10 {
            if y.len() > 0 {
                y.pop();
            }
            y.push('X').unwrap();
            assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
        }
    }
}
#[test]
fn test_surrogate_handling() {
    let mut y = Yangon::<1024>::with_capacity();
    y.push('\u{D7FF}').unwrap();
    y.push('\u{E000}').unwrap();
    assert_eq!(y.as_str().chars().count(), 2);
}
#[test]
fn test_fill_to_exact_capacity_repeatedly() {
    for size in [8, 16, 32, 64, 127, 128, 255, 256, 511, 512, 1023, 1024] {
        let mut y = Yangon::<2048>::with_capacity();
        unsafe { y.set_cap(size); }
        for _ in 0..size {
            assert!(y.push('a').is_ok());
        }
        assert!(y.push('a').is_err());
        y.clear();
        let s = "b".repeat(size);
        assert!(y.push_str(&s).is_ok());
        assert!(y.push('x').is_err());
    }
}
#[test]
fn test_extreme_push_pop_cycles() {
    let mut y = Yangon::<4096>::with_capacity();
    for cycle in 0..1000 {
        let push_count = (cycle % 50) + 1;
        for i in 0..push_count {
            if y.push(char::from_u32(97 + ((cycle + i) % 26)).unwrap()).is_err() {
                break;
            }
        }
        let pop_count = (cycle % 30) + 1;
        for _ in 0..pop_count {
            y.pop();
        }
        if cycle % 100 == 0 {
            y.clear();
        }
    }
}
#[test]
fn test_capacity_shrink_grow_cycles() {
    let mut y = Yangon::<8192>::with_capacity();
    for _ in 0..50 {
        for _ in 0..100 {
            y.push('a').ok();
        }
        y.shrink_to_fit();
        let small_cap = y.capacity();
        y.truncate(y.len() / 2);
        unsafe { y.set_cap(8192); }
        assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
    }
}
#[test]
fn test_random_operation_sequence() {
    let mut y = Yangon::<4096>::with_capacity();
    let operations = [
        "push_a", "push_emoji", "push_str", "pop", "remove", 
        "insert", "clear", "truncate", "retain"
    ];
    for i in 0..1000 {
        let op = operations[i % operations.len()];
        match op {
            "push_a" => { y.push('a').ok(); }
            "push_emoji" => { y.push('🦀').ok(); }
            "push_str" => { y.push_str("test").ok(); }
            "pop" => { y.pop(); }
            "remove" => {
                if y.len() > 0 {
                    y.remove(0);
                }
            }
            "insert" => {
                if y.capacity() > y.len() {
                    y.insert(0, 'X');
                }
            }
            "clear" => { y.clear(); }
            "truncate" => {
                if y.len() > 0 {
                    y.truncate(y.len() / 2);
                }
            }
            "retain" => {
                y.retain(|c| c != 'a');
            }
            _ => {}
        }
        assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
    }
}
#[test]
fn test_adversarial_insert_remove_pattern() {
    let mut y = Yangon::<4096>::with_capacity();
    y.push_str("0123456789").unwrap();
    for _ in 0..500 {
        let pos = y.len() / 2;
        if y.capacity() > y.len() {
            y.insert(pos, 'X');
        }
        if y.len() > 0 && pos < y.len() {
            y.remove(pos);
        }
    }
    assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
}
#[test]
fn test_every_operation_interleaved() {
    let mut y = Yangon::<4096>::from("initial");
    for i in 0..200 {
        match i % 11 {
            0 => { y.push('a').ok(); }
            1 => { y.push_str("bc").ok(); }
            2 => { y.pop(); }
            3 => if y.len() > 0 { y.remove(0); }
            4 => if y.capacity() > y.len() { y.insert(0, 'X'); }
            5 => { y.truncate(y.len().saturating_sub(1)); }
            6 => { y.retain(|c| c != 'a'); }
            7 => if y.len() > 2 { y.split_off(y.len() / 2); }
            8 => { 
                let temp = y.replace::<char, 0>('a', "A");
                y = Yangon::<4096>::from(temp.as_str());
            }
            9 => if y.len() > 5 { y.replace_range(0..2, "Z"); }
            _ => { y.clear(); y.push_str("reset").ok(); }
        }
        assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
    }
}
#[test]
fn test_replace_exponential_growth() {
    let mut y = Yangon::<8192>::from("X");
    for gen in 0..10 {
        let temp = y.replace::<char, 0>('X', "XX");
        if temp.len() > 4000 {
            break;
        }
        y = Yangon::<8192>::from(temp.as_str());
        assert!(y.as_str().chars().all(|c| c == 'X'));
        assert_eq!(y.len(), 1 << (gen + 1));
    }
}
#[test]
fn test_replace_with_unicode_expansion() {
    let mut y = Yangon::<8192>::from("aaaa");
    y = Yangon::<8192>::from(y.replace::<char, 0>('a', "🦀").as_str());
    assert_eq!(y.len(), 16); 
    y = Yangon::<8192>::from(y.replace::<char, 0>('🦀', "世界").as_str());
    assert_eq!(y.len(), 24);
}
#[test]
fn test_replace_overlapping_patterns() {
    let y = Yangon::<2048>::from("aaaa");
    let result = y.replace::<&str, 0>("aa", "a");
    assert_eq!(result.as_str(), "aa");
    let y2 = Yangon::<2048>::from("aa");
    let result2 = y2.replace::<&str, 0>("aa", "aaa");
    assert_eq!(result2.as_str(), "aaa");
}
#[test]
fn test_replace_empty_pattern_exhaustive() {
    let y = Yangon::<2048>::from("ABC");
    let result = y.replace::<&str, 0>("", "X");
    assert_eq!(result.as_str(), "XAXBXCX");
    let y2 = Yangon::<2048>::from("🦀世");
    let result2 = y2.replace::<&str, 0>("", "|");
    assert_eq!(result2.as_str(), "|🦀|世|");
}
#[test]
fn test_replace_range_all_positions() {
    let base = "0123456789";
    for start in 0..=base.len() {
        for end in start..=base.len() {
            let mut y = Yangon::<1024>::from(base);
            y.replace_range(start..end, "X");
            let expected_len = base.len() - (end - start) + 1;
            assert_eq!(y.len(), expected_len);
            assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
        }
    }
}
#[test]
fn test_replace_range_with_unicode() {
    let mut y = Yangon::<2048>::from("Hello🦀World");
    y.replace_range(5..9, "🌍");
    assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
    let mut y2 = Yangon::<2048>::from("AB");
    y2.replace_range(1..1, "🦀🦀🦀");
    assert_eq!(y2.as_str(), "A🦀🦀🦀B");
}
#[test]
fn test_retain_remove_every_nth_char() {
    for n in 1..=10 {
        let mut y = Yangon::<2048>::from("0123456789abcdefghij");
        let mut count = 0;
        y.retain(|_| {
            count += 1;
            count % n != 0
        });
        assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
    }
}
#[test]
fn test_retain_with_stateful_predicate() {
    let mut y = Yangon::<2048>::from("aAbBcCdDeEfF");
    let mut keep_next = true;
    y.retain(|_| {
        let result = keep_next;
        keep_next = !keep_next;
        result
    });
    assert_eq!(y.as_str().chars().count(), 6);
}
#[test]
fn test_retain_unicode_complexity() {
    let mut y = Yangon::<2048>::from("a🦀b世c界d!e@f#");
    y.retain(|c| c.len_utf8() > 1);
    let chars: Vec<char> = y.as_str().chars().collect();
    assert!(chars.iter().all(|c| c.len_utf8() > 1));
}
#[test]
fn test_retain_remove_all_then_add() {
    let mut y = Yangon::<1024>::from("TestString");
    y.retain(|_| false);
    assert!(y.is_empty());
    y.push_str("NewContent").unwrap();
    assert_eq!(y.as_str(), "NewContent");
}
#[test]
fn test_split_off_all_positions_with_unicode() {
    let test_str = "a🦀b世c";
    let byte_positions = [0, 1, 5, 6, 9, 10];
    for &pos in &byte_positions {
        let mut y = Yangon::<1024>::from(test_str);
        let y2 = y.split_off(pos);
        assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
        assert!(std::str::from_utf8(y2.as_str().as_bytes()).is_ok());
        assert_eq!(y.len() + y2.len(), test_str.len());
    }
}
#[test]
fn test_split_off_cascading() {
    let mut y = Yangon::<1024>::from("ABCDEFGHIJ");
    let mut parts = vec![y.clone()];
    while parts.last().unwrap().len() > 1 {
        let mut last = parts.pop().unwrap();
        let mid = last.len() / 2;
        let second = last.split_off(mid);
        parts.push(last);
        parts.push(second);
    }
    for part in &parts {
        assert!(std::str::from_utf8(part.as_str().as_bytes()).is_ok());
    }
}
#[test]
fn test_unsafe_operations_interleaved() {
    let mut y = Yangon::<2048>::with_capacity();
    unsafe {
        y.push_str_unchecked("Hello");
        let ptr = y.as_mut_ptr();
        *ptr = b'h'; 
        y.set_len(3); 
        y.push_str_unchecked(" World");
        assert_eq!(y.as_str(), "hel World");
    }
}
#[test]
fn test_from_utf8_unchecked_stress() {
    let test_cases = vec![
        vec![72, 101, 108, 108, 111], 
        vec![240, 159, 166, 128],     
        vec![228, 184, 150, 231, 149, 140], 
        vec![72, 240, 159, 166, 128, 87],
    ];
    for bytes in test_cases {
        let y = unsafe { Yangon::<1024>::from_utf8_unchecked(bytes) };
        assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
    }
}
#[test]
fn test_as_mut_ptr_complex_modifications() {
    let mut y = Yangon::<1024>::from("AAAAA");
    unsafe {
        let ptr = y.as_mut_ptr();
        for i in 0..5 {
            *ptr.add(i) = b'a' + (i as u8);
        }
    }
    assert_eq!(y.as_str(), "abcde");
}
#[test]
fn test_set_len_boundary_conditions() {
    let mut y = Yangon::<1024>::with_capacity();
    y.push_str("Hello World").unwrap();
    unsafe {
        y.set_len(0);
        assert!(y.is_empty());
        y.push_str("Test").unwrap();
        assert_eq!(y.as_str(), "Test");
        let len = y.len();
        y.set_len(len);
        assert_eq!(y.as_str(), "Test");
    }
}
#[test]
fn test_all_whitespace_types() {
    let whitespace = " \t\n\r\u{000B}\u{000C}\u{0085}\u{00A0}\u{1680}\u{2000}\u{2001}";
    let mut y = Yangon::<1024>::from(whitespace);
    y.push_str("content").unwrap();
    y.push_str(whitespace).unwrap();
    let trimmed = y.trim();
    assert_eq!(trimmed, "content");
}
#[test]
fn test_maximum_multibyte_sequence() {
    let mut y = Yangon::<2048>::with_capacity();
    for i in 0..100 {
        if y.push('🦀').is_err() {
            break;
        }
    }
    assert!(y.len() >= 400);
    assert_eq!(y.as_str().chars().count(), y.len() / 4);
}
#[test]
fn test_operations_near_capacity_exhaustion() {
    let mut y = Yangon::<100>::with_capacity();
    while y.len() < 95 {
        if y.push('a').is_err() {
            break;
        }
    }
    y.insert(0, 'X');
    y.pop();
    y.push('Y').ok();
    y.remove(0);
    y.truncate(y.len());
    assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
}
#[test]
fn test_clone_deep_stress() {
    let mut original = Yangon::<4096>::from("Base🦀Content");
    let mut clones: Vec<Yangon<4096>> = vec![];
    for _ in 0..50 {
        clones.push(original.clone());
    }
    for _ in 0..100 {
        original.push('X').ok();
        original.pop();
    }
    for clone in &clones {
        assert_eq!(clone.as_str(), "Base🦀Content");
    }
}
#[test]
fn test_iterator_exhaustion() {
    let chars: Vec<char> = (0..100)
        .filter_map(|i| char::from_u32(i))
        .collect();
    let y: Result<Yangon<65536>, yError> = chars.into_iter()
        .try_fold(Yangon::<65536>::with_capacity(), |mut acc, ch| {
            acc.push(ch)?;
            let acc: Yangon<65536> = acc;
            Ok(acc)
        });
    if let Ok(y) = y {
        assert!(y.len() > 0);
    }
}
#[test]
fn test_all_operation_pairs() {
    let mut y = Yangon::<2048>::from("initial");
    let ops = ["push", "pop", "insert", "remove", "clear"];
    for (i, op1) in ops.iter().enumerate() {
        for (j, op2) in ops.iter().enumerate() {
            y.clear();
            y.push_str("test").ok();
            match *op1 {
                "push" => { y.push('A').ok(); }
                "pop" => { y.pop(); }
                "insert" => if y.capacity() > y.len() { y.insert(0, 'B'); }
                "remove" => if y.len() > 0 { y.remove(0); }
                "clear" => { y.clear(); y.push_str("x").ok(); }
                _ => {}
            }
            match *op2 {
                "push" => { y.push('C').ok(); }
                "pop" => { y.pop(); }
                "insert" => if y.capacity() > y.len() { y.insert(0, 'D'); }
                "remove" => if y.len() > 0 { y.remove(0); }
                "clear" => { y.clear(); }
                _ => {}
            }
            assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
        }
    }
}
#[test]
fn test_exhaustive_capacity_boundaries() {
    for cap in [1, 2, 3, 4, 5, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128] {
        let mut y = Yangon::<256>::with_capacity();
        unsafe { y.set_cap(cap); }
        let mut filled = 0;
        for i in 0..cap {
            if y.push('x').is_ok() {
                filled += 1;
            } else {
                break;
            }
        }
        assert_eq!(filled, cap);
        assert!(y.push('y').is_err());
    }
}
#[test]
fn test_alternating_insert_remove_every_position() {
    let mut y = Yangon::<2048>::from("ABCDEFGHIJ");
    for round in 0..100 {
        let pos = round % y.len().max(1);
        if y.capacity() > y.len() {
            y.insert(pos, 'X');
        }
        if y.len() > 0 && pos < y.len() {
            y.remove(pos);
        }
    }
    assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
}
#[test]
fn test_thrashing_front_and_back() {
    let mut y = Yangon::<2048>::with_capacity();
    for _ in 0..500 {
        y.push('A').ok();
        y.push('B').ok();
        if y.len() > 0 {
            y.remove(0);
        }
        if y.capacity() > y.len() {
            y.insert(0, 'C');
        }
        y.pop();
    }
}
#[test]
fn test_butterfly_pattern_insertions() {
    let mut y = Yangon::<4096>::from("0123456789");
    for i in 0..100 {
        let mid = y.len() / 2;
        let offset = (i % 10) as usize;
        if y.capacity() > y.len() {
            if i % 2 == 0 {
                let pos = mid + offset.min(y.len() - mid);
                if pos <= y.len() {
                    y.insert(pos, 'L');
                }
            } else {
                let pos = mid.saturating_sub(offset);
                y.insert(pos, 'R');
            }
        }
    }
}
#[test]
fn test_sawtooth_growth_pattern() {
    let mut y = Yangon::<4096>::with_capacity();
    for cycle in 0..50 {
        for _ in 0..20 {
            y.push('G').ok();
        }
        for _ in 0..10 {
            y.pop();
        }
        assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
    }
}
#[test]
fn test_potential_integer_overflow_scenarios() {
    let mut y = Yangon::<65536>::with_capacity();
    let huge_str = "a".repeat(60000);
    y.push_str(&huge_str[..y.capacity().min(60000)]).ok();
    let remaining = y.capacity().saturating_sub(y.len());
    if remaining > 0 {
        y.insert(0, '🦀');
    }
}
#[test]
fn test_potential_off_by_one_exploits() {
    let mut y = Yangon::<1024>::with_capacity();
    let near_cap = y.capacity() - 1;
    y.push_str(&"a".repeat(near_cap)).ok();
    y.push('b').ok();
    y.pop();
    y.truncate(near_cap);
    if y.capacity() > y.len() {
        y.insert(near_cap, 'c');
    }
}
#[test]
fn test_crafted_unicode_sequences() {
    let crafted = vec![
        "a\u{0301}",            
        "\u{FEFF}invisible",      
        "a\u{200D}b",            
        "\u{202E}reversed",      
        "test\u{0000}null",       
    ];
    for input in crafted {
        let mut y = Yangon::<1024>::from(input);
        y.push('!').ok();
        y.pop();
        if y.len() > 0 {
            y.remove(0);
        }
        y.truncate(y.len());
        assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
    }
}
#[test]
fn test_long_grapheme_clusters() {
    let mut y = Yangon::<2048>::with_capacity();
    y.push('e').unwrap();
    for _ in 0..50 {
        if y.push('\u{0301}').is_err() {
            break;
        }
    }
    let char_count = y.as_str().chars().count();
    assert!(char_count > 1);
    let before = y.len();
    y.pop();
    assert!(y.len() < before);
}
#[test]
fn test_normalization_boundaries() {
    let mut y1 = Yangon::<1024>::from("é");
    let mut y2 = Yangon::<1024>::from("é"); 
    y1.push('!').unwrap();
    y2.push('!').unwrap();
    assert!(std::str::from_utf8(y1.as_str().as_bytes()).is_ok());
    assert!(std::str::from_utf8(y2.as_str().as_bytes()).is_ok());
}
#[test]
fn test_conversion_round_trips() {
    let a = "a".repeat(1000);
    let test_strings = vec![
        "Simple ASCII",
        "Unicode 🦀 混合",
        &a,
        "",
        " ",
        "\n\t\r",
    ];
    for s in test_strings {
        let y1 = Yangon::<2048>::from(s);
        let bytes = y1.clone().into_bytes();
        let y2 = Yangon::<2048>::from_utf8(bytes).unwrap();
        let s2 = y2.to_string();
        assert_eq!(s, s2);
    }
}
#[test]
fn test_from_utf8_lossy_comprehensive() {
    let invalid_sequences = vec![
        vec![0xFF],
        vec![0xFF, 0xFF],
        vec![0x80],
        vec![0xC0, 0x00], 
        vec![0xED, 0xA0, 0x80], 
        vec![72, 101, 0xFF, 108, 108, 111], 
    ];
    for bytes in invalid_sequences {
        let cow = Yangon::<1024>::from_utf8_lossy(&bytes);
        assert!(std::str::from_utf8(cow.as_bytes()).is_ok());
        assert!(cow.len() > 0 || bytes.is_empty());
    }
}
#[test]
fn test_as_ptr_stability() {
    let mut y = Yangon::<1024>::from("Hello");
    let ptr1 = y.as_ptr();
    y.truncate(4);
    let ptr2 = y.as_ptr();
    assert_eq!(ptr1, ptr2);
    y.pop();
    let ptr3 = y.as_ptr();
    assert_eq!(ptr1, ptr3);
}
#[test]
fn test_write_trait_comprehensive() {
    use std::fmt::Write;
    let mut y = Yangon::<4096>::with_capacity();
    write!(&mut y, "{}", 42).unwrap();
    write!(&mut y, " {}", 3.14159).unwrap();
    write!(&mut y, " {}", true).unwrap();
    write!(&mut y, " {:?}", "debug").unwrap();
    write!(&mut y, " {:#x}", 255).unwrap();
    assert!(y.len() > 0);
    assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
}
#[test]
fn test_write_trait_with_unicode() {
    use std::fmt::Write;
    let mut y = Yangon::<2048>::with_capacity();
    write!(&mut y, "Rust: {}", '🦀').unwrap();
    write!(&mut y, " World: {}", '世').unwrap();
    write!(&mut y, " Emoji: {}", '😀').unwrap();
    assert!(y.as_str().contains('🦀'));
    assert!(y.as_str().contains('世'));
}
#[test]
fn test_write_trait_near_capacity() {
    use std::fmt::Write;
    let mut y = Yangon::<100>::with_capacity();
    y.push_str(&"a".repeat(90)).unwrap();
    let result = write!(&mut y, "{}", "This is a long string");
}
#[test]
fn test_equality_with_various_types() {
    let y = Yangon::<1024>::from("Hello");
    assert_eq!(y, "Hello");
    assert_eq!(y.as_str(), "Hello");
    assert_eq!(&y[..], "Hello");
    assert_ne!(y, "hello");
    assert_ne!(y, "Hello ");
}
#[test]
fn test_equality_unicode_variations() {
    let y1 = Yangon::<1024>::from("café");
    let y2 = Yangon::<1024>::from("café");
    assert_eq!(y1, y2.as_str());
}
#[test]
fn test_trim_all_unicode_whitespace() {
    let unicode_spaces = vec![
        '\u{0020}', 
        '\u{00A0}', 
        '\u{1680}', 
        '\u{2000}', 
        '\u{2001}', 
        '\u{2002}', 
        '\u{2003}', 
        '\u{3000}', 
    ];
    for space in unicode_spaces {
        let mut s = String::new();
        s.push(space);
        s.push_str("content");
        s.push(space);
        let y = Yangon::<1024>::from(&s);
        assert_eq!(y.trim(), "content");
    }
}
#[test]
fn test_trim_mixed_whitespace() {
    let y = Yangon::<1024>::from("  \t\n\r  content  \t\n\r  ");
    assert_eq!(y.trim(), "content");
    let y2 = Yangon::<1024>::from("\u{00A0}\u{2000}test\u{2001}\u{3000}");
    assert_eq!(y2.trim(), "test");
}
#[test]
fn test_yangon_macro_with_various_inputs() {
    let y1 = yangon!();
    assert!(y1.is_empty());
    let y2 = yangon!("test");
    assert_eq!(y2.as_str(), "test");
    let y3 = yangon!("🦀");
    assert_eq!(y3.as_str(), "🦀");
}
#[test]
fn test_to_yangon_trait_variations() {
    let s1 = "test";
    let y1 = s1.to_yangon();
    assert_eq!(y1.as_str(), "test");
    let s2 = String::from("String");
    let y2 = s2.as_str().to_yangon();
    assert_eq!(y2.as_str(), "String");
}
#[test]
fn test_alignment_after_operations() {
    let mut y = Yangon::<1024>::with_capacity();
    for _ in 0..100 {
        y.push('a').ok();
        y.push('🦀').ok();
        y.pop();
        let ptr = y.as_ptr() as usize;
        assert_eq!(ptr % std::mem::align_of::<u8>(), 0);
    }
}
#[test]
fn test_size_and_layout_consistency() {
    let y1 = Yangon::<1024>::with_capacity();
    let y2 = Yangon::<2048>::with_capacity();
    let y3 = Yangon::<1024>::with_capacity();
    assert_eq!(
        std::mem::size_of_val(&y1),
        std::mem::size_of_val(&y3)
    );
}
#[test]
fn test_log_line_simulation() {
    let mut y = Yangon::<4096>::with_capacity();
    for i in 0..100 {
        use std::fmt::Write;
        write!(&mut y, "[INFO] Line {}: ", i).ok();
        y.push_str("Some log message 🦀\n").ok();
        if i % 25 == 0 {
            y.clear();
        }
    }
}
#[test]
fn test_json_like_construction() {
    let mut y = Yangon::<2048>::with_capacity();
    y.push('{').unwrap();
    for i in 0..20 {
        use std::fmt::Write;
        write!(&mut y, "\"key{}\":\"value{}\"", i, i).ok();
        if i < 19 {
            y.push(',').ok();
        }
    }
    y.push('}').unwrap();
    assert!(y.as_str().starts_with('{'));
    assert!(y.as_str().ends_with('}'));
}
#[test]
fn test_html_tag_building() {
    let mut y = Yangon::<2048>::with_capacity();
    y.push_str("<div class=\"container\">").unwrap();
    y.push_str("<p>Content with 🦀 emoji</p>").unwrap();
    y.push_str("</div>").unwrap();
    assert!(y.as_str().contains("🦀"));
}
#[test]
fn test_unicode_boundary_all_multibyte_sizes() {
    let mut y = Yangon::<1024>::with_capacity();
    y.push('a').unwrap();
    y.push('é').unwrap();
    y.push('世').unwrap();
    y.push('🦀').unwrap();
    assert_eq!(y.pop(), Some('🦀'));
    assert_eq!(y.pop(), Some('世'));
    assert_eq!(y.pop(), Some('é'));
    assert_eq!(y.pop(), Some('a'));
}
#[test]
fn test_insert_remove_at_every_unicode_boundary() {
    let y = Yangon::<1024>::from("a🦀世界");
    let positions = vec![0, 1, 5, 8];
    for &pos in &positions {
        let mut temp = y.clone();
        temp.insert(pos, 'X');
        assert!(temp.as_str().is_char_boundary(pos));
    }
    let mut y2 = Yangon::<1024>::from("a🦀世界");
    while !y2.is_empty() {
        let ch = y2.remove(0);
        assert!(ch.len_utf8() <= 4);
    }
}
#[test]
fn test_truncate_at_all_unicode_boundaries() {
    let s = "Hello🦀世界!";
    for i in 0..=s.len() {
        if s.is_char_boundary(i) {
            let mut y = Yangon::<1024>::from(s);
            y.truncate(i);
            assert_eq!(y.len(), i);
            assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
        }
    }
}
#[test]
fn test_split_off_at_unicode_boundaries() {
    let test_str = "ab🦀cd世界ef";
    let boundaries = [0, 1, 2, 6, 7, 8, 11, 14, 15, 16];
    for &idx in &boundaries {
        let mut y = Yangon::<1024>::from(test_str);
        let y2 = y.split_off(idx);
        assert_eq!(y.len() + y2.len(), test_str.len());
        assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
        assert!(std::str::from_utf8(y2.as_str().as_bytes()).is_ok());
    }
}
#[test]
fn test_exact_capacity_fill() {
    let mut y = Yangon::<5>::with_capacity();
    assert!(y.push_str("12345").is_ok());
    assert!(matches!(y.push('x'), Err(yError::CapacityOverflow)));
}
#[test]
fn test_multibyte_char_at_capacity_boundary() {
    let mut y = Yangon::<7>::with_capacity();
    y.push_str("abc").unwrap();
    assert!(y.push('🦀').is_ok());
    assert!(matches!(y.push('x'), Err(yError::CapacityOverflow)));
}
#[test]
fn test_capacity_off_by_one_scenarios() {
    for cap in [10, 100, 1024] {
        let mut y = Yangon::<1024>::with_capacity();
        unsafe { y.set_cap(cap); }
        let s = "a".repeat(cap);
        assert_eq!(y.push_str(&s).is_ok(), true);
        let mut y2 = Yangon::<1024>::with_capacity();
        unsafe { y2.set_cap(cap); }
        let s2 = "a".repeat(cap + 1);
        assert!(y2.push_str(&s2).is_err());
    }
}
#[test]
fn test_zero_capacity_behavior() {
    let mut y = Yangon::<1024>::with_capacity();
    unsafe { y.set_cap(0); }
    assert!(matches!(y.push('a'), Err(yError::CapacityOverflow)));
    assert!(matches!(y.push_str("a"), Err(yError::CapacityOverflow)));
}
#[test]
fn test_repeated_push_pop_cycles() {
    let mut y = Yangon::<2048>::with_capacity();
    for cycle in 0..100 {
        for i in 0..20 {
            let ch = char::from_u32(97 + ((cycle + i) % 26)).unwrap();
            y.push(ch).unwrap();
        }
        for _ in 0..20 {
            assert!(y.pop().is_some());
        }
    }
    assert!(y.is_empty());
}
#[test]
fn test_interleaved_operations() {
    let mut y = Yangon::<2048>::with_capacity();
    for i in 0..50 {
        y.push_str("test").unwrap();
        y.truncate(y.len() - 2);
        y.push('🦀').unwrap();
        y.remove(0);
        if i % 10 == 0 {
            y.clear();
        }
    }
}
#[test]
fn test_retain_with_complex_predicate() {
    let mut y = Yangon::<2048>::from("aAbBcCdDeEfF123456!@#$%^&*()🦀世界");
    y.retain(|c| !c.is_ascii_digit());
    y.retain(|c| !c.is_ascii_punctuation());
    y.retain(|c| c.is_alphabetic() || c.len_utf8() > 1);
    assert!(y.as_str().chars().all(|c| c.is_alphabetic() || c.len_utf8() > 1));
    assert_eq!(y.as_str(), "aAbBcCdDeEfF🦀世界");
}
#[test]
fn test_replace_range_edge_cases() {
    let mut y = Yangon::<1024>::from("Hello World");
    y.replace_range(0..0, ">>> ");
    assert_eq!(y.as_str(), ">>> Hello World");
    let mut y = Yangon::<1024>::from("Hello World");
    y.replace_range(11..11, " <<<");
    assert_eq!(y.as_str(), "Hello World <<<");
    let mut y = Yangon::<1024>::from("Hello World");
    y.replace_range(0..11, "Replaced");
    assert_eq!(y.as_str(), "Replaced");
    let mut y = Yangon::<1024>::from("Hello World");
    y.replace_range(5..11, "");
    assert_eq!(y.as_str(), "Hello");
}
#[test]
fn test_massive_replace_operations() {
    let mut y = Yangon::<4096>::from("aaaaaaaaaa");
    for _ in 0..10 {
        let temp = y.replace::<char, 0>('a', "bb");
        y = Yangon::<4096>::from(temp.as_str());
        if y.len() * 2 > y.capacity() {
            break;
        }
    }
}
#[test]
fn test_push_str_unchecked_safety() {
    let mut y = Yangon::<1024>::with_capacity();
    unsafe {
        y.push_str_unchecked("Hello");
        y.push_str_unchecked(" ");
        y.push_str_unchecked("World");
        y.push_str_unchecked(" 🦀");
    }
    assert_eq!(y.as_str(), "Hello World 🦀");
    assert!(std::str::from_utf8(y.as_str().as_bytes()).is_ok());
}
#[test]
fn test_from_utf8_unchecked_with_valid_data() {
    let valid_bytes = vec![
        72, 101, 108, 108, 111,  
        32,                      
        240, 159, 166, 128,      
    ];
    let y = unsafe { Yangon::<1024>::from_utf8_unchecked(valid_bytes) };
    assert_eq!(y.as_str(), "Hello 🦀");
}
#[test]
fn test_as_mut_ptr_modifications() {
    let mut y = Yangon::<1024>::from("HELLO");
    unsafe {
        let ptr = y.as_mut_ptr();
        for i in 0..5 {
            *ptr.add(i) = (*ptr.add(i)).to_ascii_lowercase();
        }
    }
    assert_eq!(y.as_str(), "hello");
}
#[test]
fn test_set_len_with_valid_utf8() {
    let mut y = Yangon::<1024>::with_capacity();
    y.push_str("Hello World").unwrap();
    unsafe {
        y.set_len(5);
    }
    assert_eq!(y.as_str(), "Hello");
    assert_eq!(y.len(), 5);
}
#[test]
fn test_from_iter_large_collection() {
    let chars: Vec<char> = (0..1000)
        .map(|i| char::from_u32(97 + (i % 26)).unwrap())
        .collect();
    let y: Yangon<2048> = chars.into_iter().collect();
    assert_eq!(y.len(), 1000);
}
#[test]
fn test_from_iter_mixed_unicode() {
    let chars = vec!['a', '🦀', '世', 'b', '界', '!', 'z'];
    let y: Yangon<1024> = chars.into_iter().collect();
    assert_eq!(y.as_str().chars().count(), 7);
}
#[test]
fn test_clone_independence() {
    let mut y1 = Yangon::<1024>::from("Original");
    let mut y2 = y1.clone();
    y1.push_str(" Modified").unwrap();
    y2.push_str(" Cloned").unwrap();
    assert_eq!(y1.as_str(), "Original Modified");
    assert_eq!(y2.as_str(), "Original Cloned");
}
#[test]
fn test_multiple_clones() {
    let original = Yangon::<1024>::from("Test🦀世界");
    let clones: Vec<_> = (0..10).map(|_| original.clone()).collect();
    for clone in &clones {
        assert_eq!(clone.as_str(), "Test🦀世界");
    }
}
#[test]
fn test_from_utf8_with_various_lengths() {
    for len in [0, 1, 10, 100, 500] {
        let bytes: Vec<u8> = (0..len).map(|i| b'a' + (i as u8 % 26)).collect();
        let result = Yangon::<1024>::from_utf8(bytes);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), len);
    }
}
#[test]
fn test_from_utf8_lossy_with_invalid_sequences() {
    let test_cases = vec![
        vec![72, 101, 0xFF, 108, 108, 111],  
        vec![0xFF, 0xFF, 72, 105],            
        vec![72, 105, 0xFF, 0xFF],            
        vec![0xC0, 0x80],                     
    ];
    for bytes in test_cases {
        let cow = Yangon::<1024>::from_utf8_lossy(&bytes);
        assert!(std::str::from_utf8(cow.as_bytes()).is_ok());
    }
}
#[test]
fn test_into_bytes_and_back() {
    let original = "Hello🦀World世界";
    let y1 = Yangon::<1024>::from(original);
    let bytes = y1.into_bytes();
    let y2 = Yangon::<1024>::from_utf8(bytes).unwrap();
    assert_eq!(y2.as_str(), original);
}
#[test]
fn test_write_trait_stress() {
    use std::fmt::Write;
    let mut y = Yangon::<2048>::with_capacity();
    for i in 0..100 {
        write!(&mut y, "{} ", i).unwrap();
    }
    assert!(y.as_str().contains("0 "));
    assert!(y.as_str().contains("99 "));
}
#[test]
fn test_display_with_special_chars() {
    let y = Yangon::<1024>::from("Test\n\t\"🦀\"");
    let displayed = format!("{}", y);
    assert_eq!(displayed, "Test\n\t\"🦀\"");
}
#[test]
fn test_debug_escaping() {
    let y = Yangon::<1024>::from("Line1\nLine2\t\"quoted\"");
    let debug_str = format!("{:?}", y);
    assert!(debug_str.contains("\\n"));
    assert!(debug_str.contains("\\t"));
}
#[test]
fn test_shrink_to_fit_various_sizes() {
    for initial_cap in [128, 256, 512, 1024] {
        for content_len in [10, 50, 100] {
            let mut y = Yangon::<2048>::with_capacity();
            unsafe { y.set_cap(initial_cap); }
            let content = "a".repeat(content_len);
            y.push_str(&content).unwrap();
            y.shrink_to_fit();
            assert!(y.capacity() >= content_len);
            assert!(y.capacity() <= initial_cap);
        }
    }
}
#[test]
fn test_empty_string_operations() {
    let mut y = Yangon::<1024>::with_capacity();
    assert_eq!(y.pop(), None);
    y.clear();
    y.truncate(0);
    y.retain(|_| true);
    let y2 = y.split_off(0);
    assert!(y2.is_empty());
}
#[test]
fn test_single_char_operations() {
    let mut y = Yangon::<1024>::from("X");
    assert_eq!(y.remove(0), 'X');
    assert!(y.is_empty());
    y.push('Y').unwrap();
    y.insert(0, 'Z');
    assert_eq!(y.as_str(), "ZY");
}
#[test]
fn test_all_whitespace_variations() {
    let whitespace_chars = [' ', '\t', '\n', '\r'];
    for &ws in &whitespace_chars {
        let mut y = Yangon::<1024>::with_capacity();
        y.push(ws).unwrap();
        y.push('a').unwrap();
        y.push(ws).unwrap();
        assert_eq!(y.trim(), "a");
    }
}
#[test]
fn test_maximum_unicode_codepoint() {
    let mut y = Yangon::<1024>::with_capacity();
    if let Some(max_char) = char::from_u32(0x10FFFF) {
        y.push(max_char).unwrap();
        assert_eq!(y.pop(), Some(max_char));
    }
}
#[test]
fn test_alternating_multibyte_sizes() {
    let mut y = Yangon::<2048>::with_capacity();
    for i in 0..100 {
        match i % 4 {
            0 => y.push('a').unwrap(),     
            1 => y.push('é').unwrap(),      
            2 => y.push('世').unwrap(),     
            _ => y.push('🦀').unwrap(),     
        }
    }
    assert_eq!(y.as_str().chars().count(), 100);
}
#[test]
fn test_replace_with_size_explosion() {
    let mut y = Yangon::<4096>::from("a");
    for _ in 0..5 {
        let temp = y.replace::<char, 0>('a', "aa");
        if temp.len() > y.capacity() / 2 {
            break;
        }
        y = Yangon::<4096>::from(temp.as_str());
    }
    assert!(y.as_str().chars().all(|c| c == 'a'));
}
#[test]
fn test_repeated_insert_at_same_position() {
    let mut y = Yangon::<2048>::from("_");
    for i in 0..50 {
        let ch = char::from_u32(97 + (i % 26)).unwrap();
        y.insert(0, ch);
    }
    assert_eq!(y.len(), 51);
}
#[test]
fn test_stress_all_ascii_printable() {
    let mut y = Yangon::<2048>::with_capacity();
    for byte in 32u8..=126 {
        y.push(byte as char).unwrap();
    }
    assert_eq!(y.len(), 95);
    assert!(y.as_str().is_ascii());
}
#[test]
fn test_retain_removes_everything_then_rebuild() {
    let mut y = Yangon::<1024>::from("HelloWorld");
    y.retain(|_| false);
    assert!(y.is_empty());
    y.push_str("Rebuilt").unwrap();
    assert_eq!(y.as_str(), "Rebuilt");
}
#[test]
fn test_basic_creation_and_capacity() {
    let y: Yangon<1024> = Yangon::with_capacity();
    assert_eq!(y.len(), 0);
    assert_eq!(y.capacity(), 1024);
    assert!(y.is_empty());
}
#[test]
fn test_push_str_basic() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    assert!(y.push_str("Hello").is_ok());
    assert_eq!(y.len(), 5);
    assert_eq!(y.as_str(), "Hello");
}
#[test]
fn test_push_str_multiple() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    assert!(y.push_str("Hello").is_ok());
    assert!(y.push_str(" ").is_ok());
    assert!(y.push_str("World").is_ok());
    assert_eq!(y.as_str(), "Hello World");
    assert_eq!(y.len(), 11);
}
#[test]
fn test_push_str_unicode() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    assert!(y.push_str("Hello").is_ok());
    assert!(y.push_str(" 世界").is_ok());
    assert!(y.push_str(" 🦀").is_ok());
    assert_eq!(y.as_str(), "Hello 世界 🦀");
}
#[test]
fn test_push_str_capacity_overflow() {
    let mut y: Yangon<10> = Yangon::with_capacity();
    assert!(y.push_str("12345").is_ok());
    assert!(matches!(y.push_str("678901"), Err(yError::CapacityOverflow)));
}
#[test]
fn test_push_str_unchecked() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    unsafe {
        y.push_str_unchecked("Hello");
        y.push_str_unchecked(" World");
    }
    assert_eq!(y.as_str(), "Hello World");
}
#[test]
fn test_push_char() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    assert!(y.push('H').is_ok());
    assert!(y.push('i').is_ok());
    assert_eq!(y.as_str(), "Hi");
}
#[test]
fn test_push_unicode_char() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    assert!(y.push('🦀').is_ok());
    assert!(y.push('世').is_ok());
    assert_eq!(y.as_str(), "🦀世");
}
#[test]
fn test_push_char_capacity_overflow() {
    let mut y: Yangon<3> = Yangon::with_capacity();
    assert!(y.push('a').is_ok());
    assert!(y.push('b').is_ok());
    assert!(y.push('c').is_ok());
    assert!(matches!(y.push('d'), Err(yError::CapacityOverflow)));
}
#[test]
fn test_pop_basic() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    y.push_str("abc").unwrap();
    assert_eq!(y.pop(), Some('c'));
    assert_eq!(y.pop(), Some('b'));
    assert_eq!(y.pop(), Some('a'));
    assert_eq!(y.pop(), None);
}
#[test]
fn test_pop_unicode() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    y.push_str("a🦀世").unwrap();
    assert_eq!(y.pop(), Some('世'));
    assert_eq!(y.pop(), Some('🦀'));
    assert_eq!(y.pop(), Some('a'));
    assert_eq!(y.pop(), None);
}
#[test]
fn test_pop_empty() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    assert_eq!(y.pop(), None);
}
#[test]
fn test_clear() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    y.push_str("Hello World").unwrap();
    assert_eq!(y.len(), 11);
    y.clear();
    assert_eq!(y.len(), 0);
    assert!(y.is_empty());
}
#[test]
fn test_truncate() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    y.push_str("Hello World").unwrap();
    y.truncate(5);
    assert_eq!(y.as_str(), "Hello");
    assert_eq!(y.len(), 5);
}
#[test]
fn test_truncate_beyond_len() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    y.push_str("Hello").unwrap();
    y.truncate(100);
    assert_eq!(y.len(), 5);
}
#[test]
fn test_remove_ascii() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    y.push_str("Hello").unwrap();
    assert_eq!(y.remove(1), 'e');
    assert_eq!(y.as_str(), "Hllo");
}
#[test]
fn test_remove_unicode() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    y.push_str("a🦀b").unwrap();
    assert_eq!(y.remove(1), '🦀');
    assert_eq!(y.as_str(), "ab");
}
#[test]
fn test_remove_multibyte_chars() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    y.push_str("Hello世界").unwrap();
    let ch = y.remove(5);
    assert_eq!(ch, '世');
    assert_eq!(y.as_str(), "Hello界");
}
#[test]
#[should_panic(expected = "Index is out of bound")]
fn test_remove_out_of_bounds() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    y.push_str("Hi").unwrap();
    y.remove(10);
}
#[test]
fn test_insert_ascii() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    y.push_str("Hllo").unwrap();
    y.insert(1, 'e');
    assert_eq!(y.as_str(), "Hello");
}
#[test]
fn test_insert_unicode() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    y.push_str("ab").unwrap();
    y.insert(1, '🦀');
    assert_eq!(y.as_str(), "a🦀b");
}
#[test]
fn test_insert_at_end() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    y.push_str("Hello").unwrap();
    y.insert(5, '!');
    assert_eq!(y.as_str(), "Hello!");
}
#[test]
fn test_insert_at_start() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    y.push_str("ello").unwrap();
    y.insert(0, 'H');
    assert_eq!(y.as_str(), "Hello");
}
#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_insert_out_of_bounds() {
    let mut y: Yangon<1024> = Yangon::with_capacity();
    y.push_str("Hi").unwrap();
    y.insert(10, 'x');
}
#[test]
#[should_panic(expected = "Capacity Overflow")]
fn test_insert_capacity_overflow() {
    let mut y: Yangon<5> = Yangon::with_capacity();
    y.push_str("1234").unwrap();
    y.insert(0, 'x');
    y.insert(0, 'y');
}
#[test]
fn test_from_str() {
    let y = Yangon::<1024>::from("Hello World");
    assert_eq!(y.as_str(), "Hello World");
    assert_eq!(y.len(), 11);
}
#[test]
fn test_from_str_unicode() {
    let y = Yangon::<1024>::from("Hello 世界 🦀");
    assert_eq!(y.as_str(), "Hello 世界 🦀");
}
#[test]
fn test_from_utf8_valid() {
    let vec = vec![72, 101, 108, 108, 111];
    let y = Yangon::<1024>::from_utf8(vec).unwrap();
    assert_eq!(y.as_str(), "Hello");
}
#[test]
fn test_from_utf8_invalid() {
    let vec = vec![0xFF, 0xFF];
    assert!(matches!(Yangon::<1024>::from_utf8(vec), Err(yError::FromUtf8Error)));
}
#[test]
fn test_from_utf8_unchecked() {
    let vec = vec![72, 101, 108, 108, 111];
    let y = unsafe { Yangon::<1024>::from_utf8_unchecked(vec) };
    assert_eq!(y.as_str(), "Hello");
}
#[test]
fn test_from_utf8_lossy_valid() {
    let bytes = b"Hello World";
    let cow = Yangon::<1024>::from_utf8_lossy(bytes);
    assert_eq!(&*cow, "Hello World");
}
#[test]
fn test_from_utf8_lossy_invalid() {
    let bytes = &[72, 101, 0xFF, 0xFF, 111];
    let cow = Yangon::<1024>::from_utf8_lossy(bytes);
    assert!(cow.contains("He"));
    assert!(cow.contains("o"));
}
#[test]
fn test_to_string() {
    let mut y = Yangon::<1024>::with_capacity();
    y.push_str("Hello World").unwrap();
    let s = y.to_string();
    assert_eq!(s, "Hello World");
}
#[test]
fn test_into_bytes() {
    let mut y = Yangon::<1024>::with_capacity();
    y.push_str("Hello").unwrap();
    let bytes = y.into_bytes();
    assert_eq!(bytes, vec![72, 101, 108, 108, 111]);
}
#[test]
fn test_as_ptr() {
    let mut y = Yangon::<1024>::with_capacity();
    y.push_str("Hello").unwrap();
    let ptr = y.as_ptr();
    unsafe {
        assert_eq!(*ptr, b'H');
        assert_eq!(*ptr.add(1), b'e');
    }
}
#[test]
fn test_as_mut_ptr() {
    let mut y = Yangon::<1024>::with_capacity();
    y.push_str("Hello").unwrap();
    let ptr = y.as_mut_ptr();
    unsafe {
        *ptr = b'h';
    }
    assert_eq!(y.as_str(), "hello");
}
#[test]
fn test_trim_spaces() {
    let y = Yangon::<1024>::from("  Hello World  ");
    assert_eq!(y.trim(), "Hello World");
}
#[test]
fn test_trim_no_spaces() {
    let y = Yangon::<1024>::from("Hello");
    assert_eq!(y.trim(), "Hello");
}
#[test]
fn test_trim_all_spaces() {
    let y = Yangon::<1024>::from("     ");
    assert_eq!(y.trim(), "");
}
#[test]
fn test_trim_empty() {
    let y = Yangon::<1024>::with_capacity();
    assert_eq!(y.trim(), "");
}
#[test]
fn test_retain_predicate() {
    let mut y = Yangon::<1024>::from("Hello World");
    y.retain(|c| c != 'l');
    assert_eq!(y.as_str(), "Heo Word");
}
#[test]
fn test_retain_all() {
    let mut y = Yangon::<1024>::from("Hello");
    y.retain(|_| true);
    assert_eq!(y.as_str(), "Hello");
}
#[test]
fn test_retain_none() {
    let mut y = Yangon::<1024>::from("Hello");
    y.retain(|_| false);
    assert_eq!(y.as_str(), "");
}
#[test]
fn test_retain_unicode() {
    let mut y = Yangon::<1024>::from("H🦀e🦀l🦀lo");
    y.retain(|c| c != '🦀');
    assert_eq!(y.as_str(), "Hello");
}
#[test]
fn test_split_off_middle() {
    let mut y = Yangon::<1024>::from("Hello World");
    let y2 = y.split_off(6);
    assert_eq!(y.as_str(), "Hello ");
    assert_eq!(y2.as_str(), "World");
}
#[test]
fn test_split_off_start() {
    let mut y = Yangon::<1024>::from("Hello");
    let y2 = y.split_off(0);
    assert_eq!(y.as_str(), "");
    assert_eq!(y2.as_str(), "Hello");
}
#[test]
fn test_split_off_end() {
    let mut y = Yangon::<1024>::from("Hello");
    let y2 = y.split_off(5);
    assert_eq!(y.as_str(), "Hello");
    assert_eq!(y2.as_str(), "");
}
#[test]
fn test_replace_str() {
    let y = Yangon::<1024>::from("Hello World");
    let y2 = y.replace::<&str, 0>("World", "Rust");
    assert_eq!(y2.as_str(), "Hello Rust");
}
#[test]
fn test_replace_char() {
    let y = Yangon::<1024>::from("Hello");
    let y2 = y.replace::<char, 0>('l', "L");
    assert_eq!(y2.as_str(), "HeLLo");
}
#[test]
fn test_replace_empty_pattern() {
    let y = Yangon::<1024>::from("Hi");
    let y2 = y.replace::<&str, 0>("", "X");
    assert_eq!(y2.as_str(), "XHXiX");
}
#[test]
fn test_replace_not_found() {
    let y = Yangon::<1024>::from("Hello");
    let y2 = y.replace::<&str, 0>("xyz", "abc");
    assert_eq!(y2.as_str(), "Hello");
}
#[test]
fn test_replace_multiple() {
    let y = Yangon::<1024>::from("aaa");
    let y2 = y.replace::<char, 0>('a', "b");
    assert_eq!(y2.as_str(), "bbb");
}
#[test]
fn test_replace_unicode() {
    let y = Yangon::<1024>::from("Hello 世界");
    let y2 = y.replace::<&str, 0>("世界", "World");
    assert_eq!(y2.as_str(), "Hello World");
}
#[test]
fn test_replace_with_closure() {
    let y = Yangon::<1024>::from("Hello123World");
    let y2 = y.replace::<fn(char)-> bool, 0>(|c: char| c.is_numeric(), "X");
    assert_eq!(y2.as_str(), "HelloXXXWorld");
}
#[test]
fn test_shrink_to_fit() {
    let mut y = Yangon::<1024>::with_capacity();
    y.push_str("Hello").unwrap();
    assert_eq!(y.capacity(), 1024);
    y.shrink_to_fit();
    assert_eq!(y.capacity(), 6);
}
#[test]
fn test_shrink_to() {
    let mut y = Yangon::<1024>::with_capacity();
    y.push_str("Hello").unwrap();
    y.shrink_to(100);
    assert_eq!(y.capacity(), 100);
}
#[test]
fn test_shrink_to_invalid() {
    let mut y = Yangon::<1024>::with_capacity();
    y.push_str("Hello").unwrap();
    let orig_cap = y.capacity();
    y.shrink_to(3);
    assert_eq!(y.capacity(), orig_cap);
}
#[test]
fn test_partial_eq() {
    let y = Yangon::<1024>::from("Hello");
    assert!(y == "Hello");
    assert!(y != "World");
}
#[test]
fn test_as_ref() {
    let y = Yangon::<1024>::from("Hello");
    let s: &str = y.as_ref();
    assert_eq!(s, "Hello");
}
#[test]
fn test_deref() {
    let y = Yangon::<1024>::from("Hello");
    assert_eq!(y.len(), 5);
}
#[test]
fn test_from_iter() {
    let y: Yangon<1024> = "Hello".chars().collect();
    assert_eq!(y.as_str(), "Hello");
}
#[test]
fn test_from_iter_unicode() {
    let y: Yangon<1024> = "🦀世界".chars().collect();
    assert_eq!(y.as_str(), "🦀世界");
}
#[test]
fn test_yangon_macro_empty() {
    let y = yangon!();
    assert_eq!(y.len(), 0);
    assert!(y.is_empty());
}
#[test]
fn test_yangon_macro_with_str() {
    let y = yangon!("Hello World");
    assert_eq!(y.as_str(), "Hello World");
}
#[test]
fn test_to_yangon_trait() {
    let s = "Hello";
    let y = s.to_yangon();
    assert_eq!(y.as_str(), "Hello");
}
#[test]
fn test_clone() {
    let y1 = Yangon::<1024>::from("Hello");
    let y2 = y1.clone();
    assert_eq!(y1.as_str(), y2.as_str());
}
#[test]
fn test_write_trait() {
    use std::fmt::Write;
    let mut y = Yangon::<1024>::with_capacity();
    write!(&mut y, "Hello {}", "World").unwrap();
    assert_eq!(y.as_str(), "Hello World");
}
#[test]
fn test_display_trait() {
    let y = Yangon::<1024>::from("Hello");
    assert_eq!(format!("{}", y), "Hello");
}
#[test]
fn test_debug_trait() {
    let y = Yangon::<1024>::from("Hello");
    assert_eq!(format!("{:?}", y), "\"Hello\"");
}
#[test]
fn test_set_len_unsafe() {
    let mut y = Yangon::<1024>::with_capacity();
    y.push_str("Hello").unwrap();
    unsafe {
        y.set_len(3);
    }
    assert_eq!(y.len(), 3);
}
#[test]
fn test_set_cap_unsafe() {
    let mut y = Yangon::<1024>::with_capacity();
    unsafe {
        y.set_cap(512);
    }
    assert_eq!(y.capacity(), 512);
}
#[test]
fn test_stress_push_pop() {
    let mut y = Yangon::<2048>::with_capacity();
    for i in 0..100 {
        y.push(char::from_u32(97 + (i % 26)).unwrap()).unwrap();
    }
    for _ in 0..100 {
        y.pop();
    }
    assert!(y.is_empty());
}
#[test]
fn test_stress_insert_remove() {
    let mut y = Yangon::<2048>::from("abcdef");
    for i in 0..10 {
        y.insert(3, char::from_u32(97 + i).unwrap());
    }
    for _ in 0..10 {
        y.remove(3);
    }
    assert_eq!(y.as_str(), "abcdef");
}
#[test]
fn test_replace_range_shrink() {
    let mut y = Yangon::<1024>::from("Hello World");
    y.replace_range(5..11, "!");
    assert_eq!(y.as_str(), "Hello!");
}
#[test]
fn test_replace_range_expand() {
    let mut y = Yangon::<1024>::from("Hi");
    y.replace_range(2..2, " there");
    assert_eq!(y.as_str(), "Hi there");
}
#[test]
fn test_replace_range_same_size() {
    let mut y = Yangon::<1024>::from("Hello");
    y.replace_range(0..5, "World");
    assert_eq!(y.as_str(), "World");
}
