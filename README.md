### Daily-dashboard

📋 Daily dashboard utilizes the installed command in your terminal. The output of a given command will be present in a separate tab. You can pass args to your command too!

#### Minimal configuration

👀 See config.example.yml:

```yaml
tabs:
  - name: "Echo"
    command: "echo"
    color: "white"
    env: []
    args:
      - "Cool"

  - name: "Tree"
    command: "tree"
    color: "green"
    env: []
    args:
      - "-L"
      - "2"
```

#### Install and launch the app

Currenly no cargo package, just the bare git repo:

```bash
$ cargo install --path .
$ daily-dashboard --config config.example.yml
```
