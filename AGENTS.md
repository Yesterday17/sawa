# AI Agent Configuration

## General Behavior

- Always write comments in English. If you find any comments which is not in English, or the English it uses looks weird, edit it immediately.
- Always respond in Chinese, but always write git commit message in English

## Project-Specific Rules

See `.cursor/rules` for complete project guidelines.

### Critical Rules Summary

1. **Always use NewType pattern for domain concepts**
   - All IDs must be newtypes (never raw UUID)
   - All domain primitives must be newtypes (Email, Username, etc.)
   - Use NonZeroU32 for quantities, never raw u32

2. **Do NOT derive PartialEq/Eq for enums and value objects**
   - Enums: use `match` or `matches!` macro instead
   - Value objects: we never compare them
   - Only IDs have PartialEq/Eq (for HashMap keys)

3. **Repository traits must be `Send + Sync + 'static`**
   - All repository traits require these bounds
   - Use `impl Future` return types

4. **Do NOT create Markdown documentation files**
   - All documentation goes in code comments
   - Use triple-slash (`///`) for public API docs
   - Use double-slash (`//`) for implementation notes

5. **Aggregates design**
   - Product and ProductVariant are separate aggregates
   - PurchaseOrder embeds PurchaseOrderItem (not IDs)
   - Sub-entities use `seq: u32`, not UUID

6. **Snapshot pattern for historical data**
   - Always snapshot data that may change (price, config)
   - History arrays: don't store from_status (derive from array)

## Code Quality Standards

- Prefer type safety over convenience
- Newtype wrappers over raw primitives
- Explicit modeling over implicit behavior
- Simple aggregates over complex hierarchies

