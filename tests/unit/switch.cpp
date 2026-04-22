// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int basic(int x) {
  int r = 0;
  switch (x) {
  case 0:
    r = 10;
    break;
  case 1:
    r = 20;
    break;
  case 2:
    r = 30;
    break;
  default:
    r = 40;
    break;
  }
  return r;
}

int stacked(int x) {
  int r = 0;
  switch (x) {
  case 1:
  case 2:
  case 3:
    r = 100;
    break;
  case 4:
  case 5:
    r = 200;
    break;
  default:
    r = 300;
    break;
  }
  return r;
}

int no_default(int x) {
  int r = -1;
  switch (x) {
  case 7:
    r = 1;
    break;
  case 8:
    r = 2;
    break;
  }
  return r;
}

int only_default(int x) {
  int r = 0;
  switch (x) {
  default:
    r = 42;
    break;
  }
  return r;
}

int default_middle(int x) {
  int r = 0;
  switch (x) {
  case 1:
    r = 1;
    break;
  default:
    r = 99;
    break;
  case 2:
    r = 2;
    break;
  }
  return r;
}

int default_first(int x) {
  int r = 0;
  switch (x) {
  default:
    r = 7;
    break;
  case 1:
    r = 1;
    break;
  case 2:
    r = 2;
    break;
  }
  return r;
}

int empty_switch(int x) {
  switch (x) {}
  return x;
}

int switch_char(char c) {
  switch (c) {
  case 'a':
    return 1;
  case 'b':
    return 2;
  case '\n':
    return 3;
  case '\0':
    return 4;
  default:
    return 0;
  }
}

enum Color { kRed, kGreen, kBlue };

int switch_enum(Color c) {
  switch (c) {
  case kRed:
    return 10;
  case kGreen:
    return 20;
  case kBlue:
    return 30;
  }
  return -1;
}

int compound_case_body(int x) {
  int r = 0;
  switch (x) {
  case 1: {
    int y = 10;
    int z = 20;
    r = y + z;
    break;
  }
  case 2: {
    int y = 100;
    r = y - 1;
    break;
  }
  default:
    r = -1;
    break;
  }
  return r;
}

int nested(int a, int b) {
  int r = 0;
  switch (a) {
  case 1:
    switch (b) {
    case 10:
      r = 11;
      break;
    case 20:
      r = 12;
      break;
    default:
      r = 13;
      break;
    }
    r += 1;
    break;
  case 2:
    r = 2;
    break;
  default:
    r = -1;
    break;
  }
  return r;
}

int switch_in_loop(int n) {
  int r = 0;
  for (int i = 0; i < n; ++i) {
    switch (i % 3) {
    case 0:
      r += 1;
      break;
    case 1:
      r += 2;
      break;
    default:
      r += 3;
      break;
    }
    r += 10;
  }
  return r;
}

int stacked_block(int x) {
  int r = 0;
  switch (x) {
  case 1:
  case 2:
  case 3: {
    int y = x * 2;
    r = y + 1;
    break;
  }
  default:
    r = 0;
    break;
  }
  return r;
}

int double_it(int v) { return v * 2; }

int switch_on_call(int x) {
  switch (double_it(x)) {
  case 0:
    return 100;
  case 2:
    return 200;
  case 4:
    return 400;
  default:
    return -1;
  }
}

int switch_on_assignment(int x) {
  int y = 0;
  int r = 0;
  switch (y = x + 1) {
  case 1:
    r = 10;
    break;
  case 2:
    r = 20;
    break;
  default:
    r = y;
    break;
  }
  return r;
}

int mixed_literal_cases(int x) {
  switch (x) {
  case -1:
    return 1;
  case 0x10:
    return 2;
  case 0xFE80:
    return 3;
  case -0xFF:
    return 4;
  default:
    return 0;
  }
}

int mixed_return_break(int x) {
  int r = -1;
  switch (x) {
  case 0:
    return 100;
  case 1:
    r = 10;
    break;
  case 2:
    return 200;
  default:
    r = 99;
    break;
  }
  return r;
}

int empty_case_with_break(int x) {
  int r = 5;
  switch (x) {
  case 1:
    break;
  case 2:
    r = 2;
    break;
  default:
    r = 9;
    break;
  }
  return r;
}

int fallthrough_one(int x) {
  int r = 0;
  switch (x) {
  case 1:
    r += 10;
    [[fallthrough]];
  case 2:
    r += 20;
    break;
  default:
    r = -1;
    break;
  }
  return r;
}

int fallthrough_chain(int x) {
  int r = 0;
  switch (x) {
  case 1:
    r += 1;
    [[fallthrough]];
  case 2:
    r += 2;
    [[fallthrough]];
  case 3:
    r += 4;
    [[fallthrough]];
  case 4:
    r += 8;
    break;
  default:
    r = -1;
    break;
  }
  return r;
}

int fallthrough_default(int x, int flag) {
  int r = 0;
  switch (x) {
  case 7:
    if (flag) {
      r = 100;
      break;
    }
    [[fallthrough]];
  default:
    r = 42;
    break;
  }
  return r;
}

int fallthrough_into_block(int x) {
  int r = 0;
  switch (x) {
  case 1:
    r += 1;
    [[fallthrough]];
  case 2: {
    int tmp = r * 10;
    r = tmp + 5;
    break;
  }
  default:
    r = -1;
    break;
  }
  return r;
}

int switch_complex_cond(int *p, int bias) {
  switch (*p + bias) {
  case 0:
    return 1;
  case 5:
    return 2;
  case 10:
    return 3;
  default:
    return 0;
  }
}

int switch_in_dowhile(int n) {
  int r = 0;
  int i = 0;
  do {
    switch (i) {
    case 0:
      r += 1;
      break;
    case 1:
      r += 10;
      break;
    default:
      r += 100;
      break;
    }
    ++i;
  } while (i < n);
  return r;
}

int continue_inside_switch(int n) {
  int r = 0;
  for (int i = 0; i < n; ++i) {
    switch (i) {
    case 0:
    case 2:
    case 4:
      continue;
    default:
      r += i;
      break;
    }
    r += 1000;
  }
  return r;
}

int case_then_default(int x) {
  int r = 0;
  switch (x) {
  case 1:
  default:
    r = 10;
    break;
  case 2:
    r = 20;
    break;
  }
  return r;
}

int default_then_case(int x) {
  int r = 0;
  switch (x) {
  case 1:
    r = 1;
    break;
  default:
  case 2:
    r = 77;
    break;
  case 3:
    r = 3;
    break;
  }
  return r;
}

int cases_and_default_stacked(int x) {
  int r = 0;
  switch (x) {
  case 1:
  case 2:
  default:
    r = 42;
    break;
  case 3:
    r = 3;
    break;
  }
  return r;
}

int stacked_with_inner_fallthrough(int x, int flag) {
  int r = 0;
  switch (x) {
  case 1:
  case 2:
  case 3:
    if (!flag) {
      r = 50;
      break;
    }
    [[fallthrough]];
  default:
    r = 999;
    break;
  }
  return r;
}

int borrow_in_condition_and_in_body(int x) {
  switch (x) {
  case 0:
    [[fallthrough]];
  default:
    return x + 1;
  }
}

int main() {
  assert(basic(0) == 10);
  assert(basic(2) == 30);
  assert(basic(99) == 40);

  assert(stacked(1) == 100);
  assert(stacked(3) == 100);
  assert(stacked(5) == 200);
  assert(stacked(9) == 300);

  assert(no_default(7) == 1);
  assert(no_default(42) == -1);

  assert(only_default(1) == 42);

  assert(default_middle(1) == 1);
  assert(default_middle(2) == 2);
  assert(default_middle(99) == 99);

  assert(default_first(1) == 1);
  assert(default_first(99) == 7);

  assert(empty_switch(5) == 5);

  assert(switch_char('a') == 1);
  assert(switch_char('b') == 2);
  assert(switch_char('\n') == 3);
  assert(switch_char('\0') == 4);
  assert(switch_char('z') == 0);

  assert(switch_enum(kRed) == 10);
  assert(switch_enum(kGreen) == 20);
  assert(switch_enum(kBlue) == 30);

  assert(compound_case_body(1) == 30);
  assert(compound_case_body(2) == 99);
  assert(compound_case_body(9) == -1);

  assert(nested(1, 10) == 12);
  assert(nested(1, 99) == 14);
  assert(nested(2, 0) == 2);
  assert(nested(3, 3) == -1);

  assert(switch_in_loop(6) == 72);

  assert(stacked_block(2) == 5);
  assert(stacked_block(9) == 0);

  assert(switch_on_call(0) == 100);
  assert(switch_on_call(1) == 200);
  assert(switch_on_call(2) == 400);
  assert(switch_on_call(99) == -1);

  assert(switch_on_assignment(0) == 10);
  assert(switch_on_assignment(1) == 20);
  assert(switch_on_assignment(9) == 10);

  assert(mixed_literal_cases(-1) == 1);
  assert(mixed_literal_cases(0x10) == 2);
  assert(mixed_literal_cases(0xFE80) == 3);
  assert(mixed_literal_cases(-0xFF) == 4);
  assert(mixed_literal_cases(7) == 0);

  assert(mixed_return_break(0) == 100);
  assert(mixed_return_break(1) == 10);
  assert(mixed_return_break(2) == 200);
  assert(mixed_return_break(99) == 99);

  assert(empty_case_with_break(1) == 5);
  assert(empty_case_with_break(2) == 2);
  assert(empty_case_with_break(9) == 9);

  assert(fallthrough_one(1) == 30);
  assert(fallthrough_one(2) == 20);
  assert(fallthrough_one(99) == -1);

  assert(fallthrough_chain(1) == 15);
  assert(fallthrough_chain(2) == 14);
  assert(fallthrough_chain(3) == 12);
  assert(fallthrough_chain(4) == 8);
  assert(fallthrough_chain(99) == -1);

  assert(fallthrough_default(7, 0) == 42);
  assert(fallthrough_default(7, 1) == 100);
  assert(fallthrough_default(99, 0) == 42);

  assert(fallthrough_into_block(1) == 15);
  assert(fallthrough_into_block(2) == 5);
  assert(fallthrough_into_block(99) == -1);

  int p_val = 5;
  assert(switch_complex_cond(&p_val, 0) == 2);
  assert(switch_complex_cond(&p_val, 5) == 3);
  assert(switch_complex_cond(&p_val, -5) == 1);
  assert(switch_complex_cond(&p_val, 99) == 0);

  assert(switch_in_dowhile(1) == 1);
  assert(switch_in_dowhile(3) == 1 + 10 + 100);

  assert(continue_inside_switch(6) == (1 + 3 + 5) + 3 * 1000);

  assert(case_then_default(1) == 10);
  assert(case_then_default(2) == 20);
  assert(case_then_default(99) == 10);

  assert(default_then_case(1) == 1);
  assert(default_then_case(2) == 77);
  assert(default_then_case(3) == 3);
  assert(default_then_case(99) == 77);

  assert(cases_and_default_stacked(1) == 42);
  assert(cases_and_default_stacked(2) == 42);
  assert(cases_and_default_stacked(3) == 3);
  assert(cases_and_default_stacked(99) == 42);

  assert(stacked_with_inner_fallthrough(1, 0) == 50);
  assert(stacked_with_inner_fallthrough(2, 1) == 999);
  assert(stacked_with_inner_fallthrough(99, 0) == 999);

  assert(borrow_in_condition_and_in_body(0) == 1);
  assert(borrow_in_condition_and_in_body(1) == 2);
  return 0;
}
