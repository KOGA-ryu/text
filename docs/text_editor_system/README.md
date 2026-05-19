# Text Editor System

This folder tracks cloud-built Text Editor feature contracts.

The cloud repo is focused on reusable Rust feature logic. Local UI composition,
Qt/binder layout, screenshots, and interaction polish happen outside this repo.

## Current Non-Goals

- custom rendering
- ropes
- piece tables
- syntax highlighting
- paste expansion
- AI prediction
- system clipboard writes from Rust

## Default Contract

- exact text export is the default
- selection behavior is explicit
- host-facing metadata is display-oriented
- cleanup receipts are preserved
