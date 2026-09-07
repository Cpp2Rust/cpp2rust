#include <cassert>

struct S;
void bump(S *p);

struct S {
  S(int a) : a_(a), self_(nullptr) {}

  S &returns_this_reference() { return *this; }

  S *returns_this_pointer() { return this; }

  S &inc() {
    a_++;
    return *this;
  }

  void set_from_this() { this->a_ = a_ + 1; }

  int get() { return a_; }

  int twice() { return this->get() * 2; }

  void link() { self_ = this; }

  void bump_me() { bump(this); }

  const S &cref() const { return *this; }

  bool is(const S *o) const { return o == this; }

  void destroy() { delete this; }

  void reset() { *this = S(0); }

  int a_;
  S *self_;
};

void bump(S *p) { p->a_++; }

struct D {
  D(int a) : a_(a) { this->a_ *= 2; }

  int a_;
};

int main() {
  S s(1);
  S &ref = s.returns_this_reference();

  ref.a_++;
  assert(s.a_ == 2);

  S *ptr = s.returns_this_pointer();
  ptr->a_++;
  assert(s.a_ == 3);

  s.inc().inc().inc();
  assert(s.a_ == 6);

  s.set_from_this();
  assert(s.a_ == 7);

  assert(s.twice() == 14);

  s.link();
  assert(s.self_ == &s);
  s.self_->a_++;
  assert(s.a_ == 8);

  s.bump_me();
  assert(s.a_ == 9);

  D d(3);
  assert(d.a_ == 6);

  const S &cr = s.cref();
  assert(cr.a_ == 9);

  S t(0);
  assert(s.is(&s));
  assert(!s.is(&t));

  S *p = new S(1);
  S *q = p->returns_this_pointer();
  q->a_++;
  assert(p->a_ == 2);
  delete p;

  S *h = new S(5);
  h->destroy();

  s.reset();
  assert(s.a_ == 0);
  assert(s.self_ == nullptr);

  return 0;
}
