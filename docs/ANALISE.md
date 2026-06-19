# Análise técnica

A aplicação segue uma arquitetura didática:

```text
Browser -> Axum -> handlers -> MemoryStore -> Askama -> Browser
```

O escopo reproduz o fluxo de produto de uma carteira simples:

1. usuário autentica;
2. acessa dashboard;
3. visualiza ativos;
4. registra compras;
5. acompanha variação.

A versão foi adaptada para ambiente sem Docker e sem CMake.
