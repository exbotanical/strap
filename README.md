# Strap: A Bootstrap Utility

Declaratively describe your project templates:

```yaml
straps:
  - name: clib
    context: ~/projects
    steps:
      - name: clone boilerplate
        run: git clone https://github.com/exbotanical/clib-boilerplate .

      - name: cleanup
        run: rm -rf .git

      - name: rename
      # STRAP_PROJECT_NAME is the project name, so if you run
      # strap clib myproject, STRAP_PROJECT_NAME will be myproject
      # STRAP_PROJECT_NAME will be interpolated in any `run` statements
        run: |
          find . -type f -exec sed -i "s/<project>/${{ STRAP_PROJECT_NAME }}/g" {} \;
          sed -i "s/<year>/$(date +%Y)/" LICENSE
  - name: npm
    context: ~/packages
    steps:
      - name: setup npm
        run: npm init -y

      - name: install static analysis deps
        run: npm i -D eslint prettier eslint-config-prettier @magister_zito/stylelint-config @magister_zito/prettier-config

      - name: create eslint config
        run:  echo -e '{\n\t"extends": [\n\t\t"@magister_zito",\n\t\t"prettier"\n\t]\n}' >| .eslintrc

      - name: create prettier config
        run:  echo '"@magister_zito/prettier-config"' > .prettierrc

      - name: create project dir
        run: mkdir src test
```

'Strap a new project:

```bash
strap clib mylib
```

```bash
cd ~/projects/mylib # begin!
```
