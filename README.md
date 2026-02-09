# Tune My Hole

A self-tuning Pi-hole companion that automatically builds lean, region-aware blocklists based on real network behavior.

Tune My Hole analyzes historical DNS traffic from Pi-hole's FTL (Faster Than Light) query database, correlates observed domains with known malicious and tracking sources, and produces a small, high-confidence local blocklist.

This project favors signal over volume and intent over automation. No list hoarding. No guesswork. Just evidence-based blocking.

## What it does

- Analyzes historical DNS queries stored in Pi-hole's long-term FTL query database
- Cross-references observed domains against curated malicious, tracking and unwanted domain sources
- Generates an optimized Pi-hole local blocklist designed to complement existing adlists

Instead of blindly blocking millions of domains, this approach produces a small, auditable rule set based on what your network actually resolves. The result is fewer breakages, faster lookups and blocking decisions you can reason about.

The analysis runs entirely offline. Query data never leaves your Pi-hole and blocking decisions remain predictable and transparent.

## Installation

### Requirements

- Pi-hole
- Read access to FTL query database

### Build

```bash
cargo build --release
```


## Usage

Tune My Hole takes a Pi-hole FTL database and an external blocklist then emits the intersection of domains that are both observed on your network and present in the blocklist.

### Dry run

Analyze the data and print the resulting domains to stdout without writing any files:

```bash
sudo tmhole \
  --db /etc/pihole/pihole-FTL.db \
  --blocklist ./oisd_big.txt \
  --dry-run
```

This is useful for inspection, auditing or piping into other tools.

### Generate an optimized blocklist

Write the resulting domain set directly to Pi-hole's local blocklist:

```bash
sudo tmhole \
  --db /etc/pihole/pihole-FTL.db \
  --blocklist ./oisd_big.txt \
  --output /etc/pihole/custom.list
```

After generation, reload Pi-hole for the changes to take effect.

Reload Pi-hole:

```bash
pihole restartdns reload
```

## Flags

| Flag          | Description                     |
| ------------- | ------------------------------- |
| `--db`        | Path to FTL query database      |
| `--blocklist` | Offline domain blocklist        |
| `--threshold` | Minimum number of hits required |
| `--output`    | Output file for Pi-hole         |
| `--dry-run`   | Print results without writing   |

## Philosophy

**Small tuned lists > big lists.**

Blindly stacking massive third-party blocklists is a common pattern and it mostly fails at what it claims to do. Huge lists consume memory, slow down lookups, increase rule churn and introduce false positives all while blocking large numbers of domains that your network will never resolve.

At the same time, keeping a tiny static list without context is just as ineffective. Blocking should be informed by actual network behavior, not guesswork or list hoarding.

Tune My Hole takes a disgustingly straightforward approach:

- Observe what your network actually queries
- Validate those domains against known bad signals
- Block only what is both relevant *and* high-confidence

The goal is not maximum block count. The goal is **correctness, performance and privacy**.
