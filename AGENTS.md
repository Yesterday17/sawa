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

2. **Repository traits must be `Send + Sync + 'static`**

   - All repository traits require these bounds
   - Use `impl Future` return types

3. **Do NOT create Markdown documentation files**

   - All documentation goes in code comments
   - Use triple-slash (`///`) for public API docs
   - Use double-slash (`//`) for implementation notes

4. **Aggregates design**

   - Product and ProductVariant are separate aggregates
   - PurchaseOrder embeds PurchaseOrderItem (not IDs)
   - All entities use UUID as primary key (including child entities)
   - Child entities are embedded in parent aggregate but have their own UUID for external references

5. **Snapshot pattern for historical data**
   - Always snapshot data that may change (price, config)
   - History arrays: don't store from_status (derive from array)
   - History records stored in separate tables with timestamp ordering

## Code Quality Standards

- Prefer type safety over convenience
- Newtype wrappers over raw primitives
- Explicit modeling over implicit behavior
- Simple aggregates over complex hierarchies
