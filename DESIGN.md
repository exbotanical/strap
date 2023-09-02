strap.yaml
```yaml
straps:
  clib:
    steps:
      # - name: create directory
      #   context: ~/repositories
      #   uses: strap_dir

      # - name: clone repo
      #   context: strap_dir # strap_dir as context value uses the last strapped directory path as context
      #   run: git clone whatever

      # - name: cleanup
      #   # omitting context runs in last step's context
      #   run: rm -rf .git

      # - name: rename
      #   run: |
      #     find . -type f -exec sed -i "s//$strap_dir/g" {} \;
      #     sed -i "s/<year>/$(date +%Y)/" LICENSE

```
