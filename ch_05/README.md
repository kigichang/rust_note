# Ch 05 References

References come in two kinds:

- Shared Reference: let read but not modify its referent. Shared Reference are __COPY__.
- Mutable Reference: let read and modify value. But may not have any other references of any sort to that value at the same time. Mutable Reference are __NOT Copy__.
  - Multiple readers and single writer.

## References Are __NEVER__ Null

- Option