# Rust Server Monitoring API

Uma API de monitoramento de servidor desenvolvida em Rust que fornece métricas em tempo real sobre o desempenho do sistema.

## Características

- 🚀 **Alto Desempenho**: Construída com Rust para máxima eficiência - Cerca de 2.3MB de memoria
- 📊 **Métricas em Tempo Real**: Monitore CPU
- 🔌 **API RESTful**: Interface simples e intuitiva
- 📈 **Coleta de Dados**: Histórico de métricas para análise

## Tecnologias Utilizadas

- **Rust** - Linguagem de programação principal
- **Tokio** - Runtime assíncrono
- **Serde** - Serialização de dados
- **Reqwest** - Cliente HTTP

## Instalação

```bash
git clone https://github.com/seu-usuario/server_monitoring_api.git
cd server_monitoring_api
cargo build --release
```

## Como Usar

```bash
cargo run
```

A API estará disponível em `http://localhost:3000`

## Endpoints

- `GET /memory` - Obter informações da memória do sistema

  - Retorna: `{"total_memory": number, "used_memory": number, "free_memory": number}`

- `GET /cpu` - Obter informações da CPU do sistema

  - Retorna: `{"total_cpus": number, "total_cpu_usage": number, "cores_usage": array}`
  - Também salva os dados no banco de dados

- `GET /uptime` - Obter tempo de atividade do sistema

  - Retorna: `{"data": "string"}`

- `GET /cpu/history` - Obter histórico de uso da CPU
  - Retorna: `{"data": [{"id": number, "total_cpus": number, "total_cpu_usage": number, "cores_usage": array, "created_at": "string"}]}`

## Contribuição

Contribuições são bem-vindas! Por favor, abra uma issue ou envie um pull request.
