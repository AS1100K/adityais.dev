# `tool-contributing`

> [!NOTE]
> Please refer to [`tools/README.md`](../README.md) before this.

## Output

This tool generates two files: `prs.json` and `stars.json`.

### `prs.json`

_Example_

```json
[
  {
    "html_url": "https://github.com/AS1100K/adityais.dev/pull/4",
    "title": "Add tools for getting contributing data",
    "number": 4,
    "owner_repo": ["AS1100K", "adityais.dev"],
    "updated_at": "2025-02-17T16:06:07Z",
    "state": "Open",
    "owner_avatar_url": "https://avatars.githubusercontent.com/u/117935160?v=4"
  }
]
```

_Types_

- `html_url`: Direct url to the Pull Request
- `title`: Title of the PR
- `number`: Number of the PR
- `owner_repo`: Array of repository owner and repo respectively
- `created_at`: PR created at
- `state`:
  - `Open`: The PR is open
  - `Draft`: The PR is marked as draft
  - `Closed`: The PR is closed with unmerged commits
  - `Merged`: The PR is merged
- `owner_avatar_url`: PR Author's GitHub Avatar. For resizing pass `s={width}` param

### `repo-info.json`

_Example_

```json
{
  "AS1100K/cargo-wiki": {
    "stars": 3,
    "avatar_url": "https://avatars.githubusercontent.com/u/117935160?v=4"
  }
}
```

_Field Syntax: `<REPO_OWNER>/<REPO_NAME>`_
