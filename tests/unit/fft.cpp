// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <cmath>
#include <memory>

struct Complex {
  double re;
  double img;
};

Complex Product(Complex z1, Complex z2) {
  double ac = z1.re * z2.re;
  double bd = z1.img * z2.img;
  double ad = z1.re * z2.img;
  double bc = z1.img * z2.re;
  return {ac - bd, ad + bc};
}

Complex Sum(Complex z1, Complex z2) {
  double ac = z1.re + z2.re;
  double bd = z1.img + z2.img;
  return {ac, bd};
}

Complex Neg(Complex z1) { return {-z1.re, -z1.img}; }

std::unique_ptr<Complex[]> fft(std::unique_ptr<Complex[]> &a, int N) {
  std::unique_ptr<Complex[]> y = std::make_unique<Complex[]>(N);
  if (N == 1) {
    y[0] = {a[0].re, a[0].img};
    return y;
  }
  std::unique_ptr<Complex[]> w = std::make_unique<Complex[]>(N);
  for (int i = 0; i < N; i++) {
    double alpha = -2 * M_PI * i / N;
    w[i] = {cos(alpha), sin(alpha)};
  }
  std::unique_ptr<Complex[]> A0 = std::make_unique<Complex[]>(N / 2);
  std::unique_ptr<Complex[]> A1 = std::make_unique<Complex[]>(N / 2);
  for (int i = 0; i < N / 2; i++) {
    A0[i] = {a[i * 2].re, a[i * 2].img};
    A1[i] = {a[i * 2 + 1].re, a[i * 2 + 1].img};
  }
  std::unique_ptr<Complex[]> y0 = fft(A0, N / 2);
  std::unique_ptr<Complex[]> y1 = fft(A1, N / 2);
  for (int k = 0; k < N / 2; k++) {
    Complex yk = Sum(y0[k], Product(w[k], y1[k]));
    y[k] = {yk.re, yk.img};
    Complex yk_n2 = Sum(y0[k], Neg(Product(w[k], y1[k])));
    y[k + N / 2] = {yk_n2.re, yk_n2.img};
  }
  return y;
}

int main() {
  int N = 4;
  std::unique_ptr<Complex[]> a = std::make_unique<Complex[]>(N);
  for (int i = 0; i < N; i++)
    a[i] = {(double)i + 1, 0};
  std::unique_ptr<Complex[]> b = fft(a, N);
  std::unique_ptr<int[]> reals = std::make_unique<int[]>(N);
  std::unique_ptr<int[]> imgs = std::make_unique<int[]>(N);
  for (int i = 0; i < N; ++i) {
    reals[i] = round(b[i].re);
    imgs[i] = round(b[i].img);
  }
  assert(reals[0] == 10 && imgs[0] == 0 && reals[1] == -2 && imgs[1] == 2 &&
         reals[2] == -2 && imgs[2] == 0 && reals[3] == -2 && imgs[3] == -2);
  return 0;
}
