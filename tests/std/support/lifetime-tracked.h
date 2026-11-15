#pragma once

struct LifetimeTracked {
  static inline int ctorCount = 0;
  static inline int copyCount = 0;
  static inline int dtorCount = 0;
  static inline int useCount = 0;

  mutable int uses = 0;

  LifetimeTracked() { ++ctorCount; }
  LifetimeTracked(const LifetimeTracked &) { ++copyCount; }
  ~LifetimeTracked() { ++dtorCount; }
  void use() const {
    ++uses;
    ++useCount;
  }
};
