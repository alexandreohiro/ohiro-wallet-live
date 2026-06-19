# Ohiro Wallet Live

Carteira de investimentos em Rust, construída como projeto prático de Axum + Askama, com autenticação por sessão JWT HS256 e armazenamento em memória.

Esta versão foi preparada para rodar em ambiente restrito de Windows/Git Bash, sem Docker, sem CMake e sem dependências nativas como BoringSSL.

## Objetivo

O projeto implementa uma carteira de investimentos com:

- login e cadastro;
- sessão em cookie HttpOnly;
- dashboard SSR com Askama;
- cadastro de ativos;
- registro de compras;
- histórico por ativo;
- cálculo de variação financeira;
- API REST básica protegida por `x-api-key`;
- valores monetários em centavos, evitando `f32/f64` para dinheiro.

## Como rodar

```bash
cp .env.example .env
cargo run
```

Acesse:

```text
http://127.0.0.1:3000
```

Usuário demo:

```text
username: alexandre
password: rust123
```


## Checklist de validação local

Antes de entregar, rode:

```bash
cargo fmt --check
cargo test
cargo run
```

O arquivo `.env` não deve ser versionado. Use `.env.example` como modelo de configuração local.

## API

```bash
curl http://127.0.0.1:3000/api/assets -H "x-api-key: dev-secret"
```

Criar ativo:

```bash
curl -X POST http://127.0.0.1:3000/api/assets \
  -H "Content-Type: application/json" \
  -H "x-api-key: dev-secret" \
  -d '{"name":"Tesouro","unit_value_cents":10000}'
```

## Regra financeira

A variação segue:

```text
change = (unit_value_atual - bought_for) × quantity
```

Internamente:

- dinheiro: centavos (`i64`);
- quantidade: milésimos (`i64`).

## Nota de autoria

Este projeto é uma implementação própria para estudo e portfólio, inspirada pelo escopo de uma carteira de investimentos em Rust. Não reproduz integralmente código, layout proprietário ou assets da aula.
## Nota workpc

Esta versão evita dependências nativas como `boring-sys`, `cmake`, `sqlx` e Docker por padrão, para rodar em ambiente Windows/Git Bash sem permissão de administrador.

Se estiver usando `w64devkit` e Rust GNU, aplique o ajuste de linker antes de rodar:

```bash
export PATH="/d/w64devkit/bin:$PATH"
mkdir -p "$HOME/gcc-libs"
cp "$(gcc -print-file-name=libgcc.a)" "$HOME/gcc-libs/libgcc_eh.a"
export RUSTFLAGS="-L native=$HOME/gcc-libs -C link-arg=-L$HOME/gcc-libs"
```

