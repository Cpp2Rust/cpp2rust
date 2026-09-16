struct Level0 {
  struct Level1_1 {
    int x1;
    struct Level2_1 {
      int x1;
      struct Level3_1 {
        int x1;
      };
      struct Level3_2 {
        int x1, x2;
      };
    };
  };
  struct Level1_2 {
    int x1, x2;
  };
};

int main() {
  [[maybe_unused]] Level0::Level1_1 x1{0};
  [[maybe_unused]] Level0::Level1_2 x2{1, 2};
  [[maybe_unused]] Level0::Level1_1::Level2_1 x3{3};
  [[maybe_unused]] Level0::Level1_1::Level2_1::Level3_1 x4{4};
  [[maybe_unused]] Level0::Level1_1::Level2_1::Level3_2 x5{5, 6};
  return 0;
}
