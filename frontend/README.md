# ToDo Frontend

ToDoアプリのフロントエンドディレクトリです。


## 開発環境の構築
### 前提条件
下記環境が構築できていることを前提とします。
- WSL2
- Docker
- Devcontainer

### `.env` の作成

```sh
cp .env.example .env.development.local
```

### `devcontainer.json` の作成

```sh
cp devcontainer.json.example devcontainer.json
```

## 実行

### 開発（ホットリロード）

```sh
pnpm dev
```

### 本番

```sh
pnpm build
pnpm start
```