# Entrega DIO — Ohiro Wallet Live

Projeto: carteira de investimentos em Rust.

## Implementado

- Servidor Axum.
- Frontend SSR com Askama.
- Login e cadastro.
- Sessão por cookie HttpOnly com token assinado via HMAC-SHA256.
- Armazenamento em memória para execução local sem Docker.
- Dashboard de ativos.
- Registro de compras.
- Histórico de compras por ativo.
- Cálculo de variação financeira.
- API REST básica com `x-api-key`.

## Decisões técnicas

- Dinheiro representado como centavos (`i64`).
- Quantidade representada como milésimos (`i64`).
- Evitado uso de `f32/f64` para cálculo financeiro.
- Versão local sem PostgreSQL para permitir execução em ambiente restrito sem Docker/admin.

## Como validar

```bash
cp .env.example .env
cargo test
cargo run
```

A aplicação sobe em `http://127.0.0.1:3000` usando a configuração local do `.env.example`.

## Declaração

Estou ciente dos termos de uso da plataforma e de que o projeto pode ser visto por recrutadores. A implementação foi organizada e escrita como estudo próprio, evitando cópia direta de código ou material proprietário.
