# Test Suite

```bash
# Run all tests
ninja check

# Run only the unit tests
ninja check-unit

# Run libcc2rs unit tests
ninja check-libcc2rs

# Run libcc2rs-macros unit tests
ninja check-libcc2rs-macros

# Regenerate expected output for unit tests after intentional changes
REPLACE_EXPECTED=1 ninja check-unit
```
