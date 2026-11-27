# Codex worker (alpha, primary owner only)

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

This is an independent Codex CLI worker, not a relay into a ChatGPT
conversation. The user must authorize Codex CLI on the isolated worker
and provision GitHub CLI publication rights for Base-Matter only.
The bot has no GitHub write token and no OpenAI API key.

## Authorization and boundaries

- Vault-Matter alpha must support the authenticated primary-owner check.
- WhatsApp requests use the existing signed, replay-protected IPC, slot 1
  PN/LID and private chat only. Socket owners and secondary global owners
  do not gain Codex access.
- The Linux worker runs as a distinct unprivileged account and verifies
  the bot UID with SO_PEERCRED; the socket is not exposed over TCP.
- Install the worker script and unit from a reviewed revision as root-owned
  files. Do not execute the worker as the WhatsApp user, root, or an account
  with Vault access.
- Configure a dedicated OpenAI login and a narrowly scoped GitHub
  publication identity under the worker account. Do not copy bot keys,
  Vault secrets or broader GitHub credentials. Model sandbox read access
  is not equivalent to a separate credential vault; use an isolated
  disposable worker environment, and do not put unrelated credentials
  there. Network permissions must remain restricted for agent commands.
- Codex works in a separate clone, workspace-write, no approval escalation.
  It cannot change the live checkout through this bridge. Prompts are
  private in the local queue, not posted to GitHub. Do not send secrets.
- Publication is a branch and PR to alpha, never an automatic merge.
  Review CI and the diff before merging. A merged PR is the only worker
  state that instructs the owner to execute =/>update.
- One local tracked task per bot; the provider polls every 20 seconds and
  retries notifications when connected. The worker persists states and
  does not repeat a task interrupted during execution.

## Host provisioning (Debian/Linux; operator action)

Install current Codex CLI and GitHub CLI on the isolated host. Verify
their paths with `command -v codex gh` and ensure the service PATH
contains them. Provision the separate account and IPC group:

```bash
sudo groupadd -f matter-codex-ipc
id matter-codex >/dev/null 2>&1 || sudo useradd --system --home-dir /var/lib/matter-codex --gid matter-codex-ipc --shell /usr/sbin/nologin matter-codex
sudo usermod -aG matter-codex-ipc "$(id -un)"
sudo install -d -m 0700 -o matter-codex -g matter-codex-ipc /var/lib/matter-codex
sudo install -d -m 0755 /etc/matter-codex /usr/local/libexec
printf 'MATTER_CODEX_BOT_UID=%s\n' "$(id -u)" | sudo tee /etc/matter-codex/worker.env >/dev/null
sudo chmod 0600 /etc/matter-codex/worker.env
sudo install -m 0755 scripts/matter-codex-worker.py /usr/local/libexec/matter-codex-worker.py
sudo install -m 0644 deploy/systemd/matter-codex-worker.service /etc/systemd/system/matter-codex-worker.service
sudo systemctl daemon-reload
```

Authorize Codex and gh explicitly as the worker, using their supported
interactive login flows; never paste tokens into WhatsApp or this repository.
Use `sudo -u matter-codex -H codex login` and
`sudo -u matter-codex -H gh auth login`, then
`sudo -u matter-codex -H gh auth setup-git` if the chosen scoped GitHub
authorization supports Git HTTPS. Confirm permissions and sandbox operation.
After setup, start the service:

```bash
sudo systemctl enable --now matter-codex-worker.service
sudo systemctl status matter-codex-worker.service --no-pager
```

The existing login/tmux server may need to be restarted after the group
change (preserve the WA session; do not delete any DB). Verify with
`id` and `stat /run/matter-codex/worker.sock`. Do not loosen socket
or home permissions to bypass a denied access.

## WhatsApp

`=/>codex` — syntax; `=/>codex <instruction>` — submit;
`=/>codex status` — current task. States: queued, running, review,
ready, failed, no changes, closed. A failed service is not a successful
development task. The worker does not restart or update the bot.
Use `=/>update` after the ready notice and `=/>restart` to activate
compiled code, according to the selected update policy.

## Validation

Rust compilation and Clippy, worker integration with real Codex/GitHub
authorization, CI, and actual WhatsApp delivery must be checked before
claiming deployment complete. The worker does not auto-merge or certify
a PR merely because Codex finished.
