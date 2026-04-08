# 🛡️ AI-Powered Smart Contract Security CLI

An intelligent CLI tool designed to automatically analyze Solidity projects (Hardhat or Foundry) and detect potential vulnerabilities using AI-driven reasoning combined with programmatic code analysis.

The tool operates as an autonomous agent equipped with file system capabilities such as reading, editing, and navigating project files. It deeply understands the structure and logic of smart contracts to identify security risks, edge cases, and unsafe patterns that traditional static analyzers may overlook.

Instead of only reporting issues, the system goes a step further by generating reproducible test cases that demonstrate how each vulnerability can be exploited. These tests serve as concrete proof-of-concepts, helping developers quickly understand and validate the risks.

## 🔍 Key Features

- **Automated Vulnerability Detection**
  Identifies common and complex smart contract vulnerabilities through contextual code analysis.

- **AI-Driven Code Reasoning**
  Goes beyond static rules by understanding contract behavior, interactions, and edge cases.

- **Exploit Test Generation**
  Automatically creates tests that trigger detected vulnerabilities, reducing false positives and improving developer trust.

- **Project-Aware Analysis**
  Works across entire codebases, handling imports, dependencies, and multi-contract interactions.

- **Comprehensive Security Reports**
  Generates clear, structured reports with vulnerability descriptions, severity, and proof-of-concept tests.

## 🎯 Objective

Provide developers with a fast, automated "pre-audit" tool that bridges the gap between static analysis and manual security reviews — improving code quality and reducing the cost and time required for professional audits.

## ⚠️ Scope

This tool focuses strictly on detection, validation, and reporting. It does not automatically fix vulnerabilities, ensuring developers retain full control over their codebase and security decisions.

## 🚀 Installation

```bash
cargo install auditra
```

Or build from source:

```bash
git clone https://github.com/ayoubbuoya/auditra.git
cd auditra
cargo build --release
```

## 📖 Usage

### Basic Usage

```bash
auditra analyze <project-path>
```

### Example Commands

```bash
# Analyze a Hardhat project
auditra analyze ./my-hardhat-project

# Analyze a Foundry project
auditra analyze ./my-foundry-project

# Generate report with custom output
auditra analyze ./my-project --output report.json

# Run with specific severity threshold
auditra analyze ./my-project --min-severity high
```

## 📋 Supported Vulnerability Types

- Reentrancy attacks
- Integer overflow/underflow
- Access control issues
- Unchecked external calls
- Timestamp dependence
- Front-running susceptibility
- Logic errors and edge cases
- Gas optimization issues
- And more...

## 📊 Report Format

The tool generates comprehensive reports including:

- **Vulnerability Summary**: High-level overview of detected issues
- **Detailed Analysis**: In-depth explanation of each vulnerability
- **Severity Assessment**: Critical, High, Medium, Low classifications
- **Proof-of-Concept Tests**: Reproducible test cases for each finding
- **Remediation Recommendations**: Suggested fixes and best practices

## 🔧 Project Requirements

The tool works with projects that meet these criteria:

- **Hardhat** or **Foundry** based Solidity projects
- Solidity ^0.8.0 (or compatible versions)
- Standard project structure with `src/` or `contracts/` directory

## ⚠️ Disclaimer

This tool is designed for educational and development purposes. While it aims to identify potential vulnerabilities, it does not guarantee complete security coverage. Always combine this tool with professional security audits before deploying to mainnet.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Inspired by the need for accessible smart contract security tools

**Note**: This is a development tool. Always conduct thorough testing and professional audits before deploying smart contracts to production environments.
