# TODOs

- [ ] Atomic steps (e.g. lockfiles for file descriptors)
- [ ] Rewind steps on failure
  - [ ] Configurable - rewind failing step or all steps? Or list of steps to rewind on failure?
  e.g.
  ```
  - name: clone
    ...
  - name: setup
    ...
  - name: cleanup
    run: rm -rf .git
    rewind_steps_on_failure:
      - cleanup
      - setup
  ```
