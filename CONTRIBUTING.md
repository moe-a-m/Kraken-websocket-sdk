# Contributing to Kraken SDK

Thank you for your interest in contributing to the Kraken SDK! This document provides guidelines and information for contributors.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- A Kraken account (for testing private API features)

### Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-org/kraken-sdk-rust.git
   cd kraken-sdk-rust
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Run tests**
   ```bash
   cargo test
   ```

4. **Run examples**
   ```bash
   cargo run --example ticker
   ```

## Development Guidelines

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Ensure no clippy warnings: `cargo clippy -- -D warnings`
- Use meaningful variable and function names
- Add documentation for public APIs

### Testing

- Write unit tests for new functionality
- Add integration tests for API interactions
- Ensure all tests pass: `cargo test`
- Test examples work correctly

### Documentation

- Document all public APIs with rustdoc comments
- Update README.md for significant changes
- Add examples for new features
- Update CHANGELOG.md

## Contribution Process

### 1. Issue First

- Check existing issues before creating new ones
- Discuss major changes in issues before implementing
- Use issue templates when available

### 2. Fork and Branch

```bash
git checkout -b feature/your-feature-name
```

### 3. Make Changes

- Keep commits focused and atomic
- Write clear commit messages
- Follow conventional commit format when possible

### 4. Test Thoroughly

```bash
# Run all tests
cargo test

# Check formatting
cargo fmt --check

# Check for warnings
cargo clippy -- -D warnings

# Test examples
cargo run --example ticker
```

### 5. Submit Pull Request

- Fill out the PR template completely
- Link related issues
- Ensure CI passes
- Request review from maintainers

## Types of Contributions

### Bug Fixes
- Fix issues in existing functionality
- Add regression tests
- Update documentation if needed

### New Features
- Implement new Kraken API endpoints
- Add new event types
- Enhance performance or reliability

### Documentation
- Improve API documentation
- Add usage examples
- Fix typos or unclear explanations

### Performance
- Optimize hot paths
- Reduce memory allocations
- Improve parsing efficiency

## Code Review Process

1. **Automated Checks**: CI must pass
2. **Maintainer Review**: At least one maintainer approval
3. **Testing**: Verify examples and tests work
4. **Documentation**: Ensure docs are updated

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create release PR
4. Tag release after merge
5. Publish to crates.io

## Community Guidelines

### Be Respectful
- Use inclusive language
- Be constructive in feedback
- Help newcomers learn

### Be Professional
- Focus on technical merit
- Avoid personal attacks
- Maintain project quality standards

## Getting Help

- **Documentation**: Check [DOCUMENTATION.md](DOCUMENTATION.md)
- **Examples**: See `examples/` directory
- **Issues**: Create GitHub issue for bugs/questions
- **Discussions**: Use GitHub Discussions for general questions

## Recognition

Contributors will be:
- Listed in release notes
- Added to contributors list
- Credited in documentation

Thank you for contributing to the Kraken SDK! 