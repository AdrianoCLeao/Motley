# MOTLEY: Model Of Torque, Load, and Engine Yield

## 1. Visão Geral do Projeto
MOTLEY é um simulador de motores baseado em Rust que permite aos usuários carregar modelos 3D de motores e explorar sua dinâmica. Ele oferece análises em tempo real de torque, resposta à carga e rendimento do motor, combinadas com simulações sonoras. O projeto será open-source, visando robustez, performance e extensibilidade.

## Estrutura de pastas
A estrutura de pastas esperada do projeto é esta

motley/
- `src/`: Contém o código-fonte principal do projeto em Rust.
  - `src/main.rs`: Ponto de entrada da aplicação.
  - `src/engine/`: Módulo central da engine.
    - `src/engine/mod.rs`: Declaração do módulo.
    - `src/engine/rendering.rs`: Lógica de renderização 3D.
    - `src/engine/physics.rs`: Simulação de física (torque, carga).
    - `src/engine/audio.rs`: Simulação de som.
    - `src/engine/utils.rs`: Funções auxiliares.
  - `src/gui/`: Interface gráfica do usuário.
    - `src/gui/mod.rs`: Declaração do módulo.
    - `src/gui/window.rs`: Janela principal da GUI.
    - `src/gui/components.rs`: Componentes visuais.
  - `src/models/`: Manipulação de modelos 3D.
    - `src/models/mod.rs`: Declaração do módulo.
    - `src/models/loader.rs`: Carregamento de modelos (OBJ, GLTF).
    - `src/models/parser.rs`: Parsing e validação de modelos.
    - `src/models/serializer.rs`: Exportação de resultados.
  - `src/tests/`: Testes unitários e de integração.
    - `src/tests/integration.rs`: Testes de integração.
    - `src/tests/performance.rs`: Benchmarks de desempenho.
    - `src/tests/unit/`: Testes unitários específicos.
      - `src/tests/unit/physics.rs`: Testes para simulação física.
      - `src/tests/unit/rendering.rs`: Testes para renderização.
      - `src/tests/unit/audio.rs`: Testes para simulação de som.
- `assets/`: Recursos do projeto.
  - `assets/models/`: Modelos 3D de exemplo.
  - `assets/textures/`: Texturas usadas nos modelos.
  - `assets/audio/`: Sons de exemplo.
  - `assets/shaders/`: Shaders usados na renderização.
- `docs/`: Documentação do projeto.
  - `docs/api/`: Documentação das APIs internas.
  - `docs/user-guide.md`: Guia do usuário final.
  - `docs/development.md`: Guia para desenvolvedores.
- `examples/`: Exemplos de uso da engine.
  - `examples/minimal.rs`: Exemplo básico.
  - `examples/advanced.rs`: Exemplo avançado com física e som.
  - `examples/custom_plugin.rs`: Exemplo de extensão com plugin.
- `scripts/`: Scripts auxiliares.
  - `scripts/build.sh`: Script de build.
  - `scripts/run.sh`: Script para rodar o projeto.
  - `scripts/test.sh`: Script para executar os testes.
- `.github/`: Configurações para GitHub.
  - `.github/workflows/`: Configurações do GitHub Actions.
    - `.github/workflows/build.yml`: Pipeline de build.
    - `.github/workflows/test.yml`: Pipeline de testes.
  - `.github/ISSUE_TEMPLATE.md`: Modelo para issues.
- `Cargo.toml`: Configuração do projeto e dependências.
- `README.md`: Documentação inicial do projeto.
- `LICENSE`: Licença do projeto.
- `CONTRIBUTING.md`: Regras de contribuição para o projeto.
- `.gitignore`: Arquivos a serem ignorados pelo Git.


## 2. Documento de Requisitos

### Funcionalidades Principais:
1. **Renderização 3D:**
   - Suporte a modelos 3D em formatos comuns (e.g., OBJ, GLTF, FBX).
   - Iluminação básica e texturização.
   - Visualização interativa com controles de câmera e zoom.

2. **Simulação de Dinâmica:**
   - Cálculo de torque e resposta à carga.
   - Simulação de rendimento do motor com base em entrada de parâmetros físicos.
   - Exportação de resultados em formato JSON/CSV.

3. **Simulação de Som:**
   - Geração de sons em tempo real com base na dinâmica do motor.
   - Ajuste de parâmetros como RPM e carga para alterar os sons simulados.

4. **Interface Gráfica do Usuário (GUI):**
   - Interface intuitiva para carregar modelos e configurar simulações.
   - Exibição de gráficos em tempo real para torque, carga e rendimento.

5. **Extensibilidade:**
   - Permitir que desenvolvedores adicionem novos cálculos e componentes via plugins.

### Requisitos Não-Funcionais:
- **Desempenho:** Processamento em tempo real para simulações dinâmicas.
- **Portabilidade:** Suporte para Windows, macOS e Linux.
- **Segurança:** Garantir estabilidade contra entradas malformadas.
- **Escalabilidade:** Suporte para modelos 3D complexos sem comprometimento do desempenho.

---

## 3. Tecnologias Utilizadas

### Linguagem:
- **Rust:** Pela performance, segurança de memória e suporte a paralelismo.

### Bibliotecas e Ferramentas:
1. **Renderização 3D:**
   - `wgpu` (para gráficos GPU-agnósticos).
   - `nannou` (framework para visualizações interativas).

2. **Física:**
   - `rapier` (motor de física 2D/3D).

3. **Simulação de Som:**
   - `rodio` (biblioteca de áudio).

4. **Interface Gráfica:**
   - `egui` (para GUI leve e performática).

5. **Conversão e Manipulação de Modelos 3D:**
   - `assimp` (para importação de modelos 3D).

6. **Exportação de Dados:**
   - `serde` e `csv` (para serialização de dados).

7. **Gerenciamento de Projetos:**
   - `cargo` (gerenciador de pacotes e builds do Rust).

8. **Testes e Benchmarking:**
   - `criterion` (benchmarking).
   - `mockall` (mocking para testes).

---

## 4. Planejamento e Cronograma

### Mês 1: Configuração Inicial (30/01/2024)
- Estruturar repositório GitHub com documentação inicial. ✅
- Configurar ambiente de desenvolvimento.
- Implementar um protótipo básico de renderização 3D usando `wgpu`.
- Entregável: Protótipo de renderização de um modelo 3D simples.

### Mês 2: Física Básica (30/02/2024)
- Integrar a biblioteca `rapier` para simulação física.
- Implementar cálculos básicos de torque e carga.
- Entregável: Simulação física rudimentar com visualização gráfica.

### Mês 3: Simulação de Som (30/03/2024)
- Integrar a biblioteca `rodio` para geração de áudio.
- Sincronizar sons com a dinâmica do motor.
- Entregável: Protótipo de simulação sonora.

### Mês 4: Interface Gráfica (30/04/2024)
- Implementar GUI usando `egui`.
- Adicionar controles para carregar modelos e configurar simulações.
- Entregável: Interface básica funcional.

### Mês 5: Análise e Exportação de Dados (30/05/2024)
- Implementar geração de gráficos em tempo real (e.g., torque vs. tempo).
- Adicionar suporte para exportação de dados.
- Entregável: Gráficos e exportação de dados de simulação.

### Mês 6: Estabilidade e Extensibilidade (30/06/2024)
- Melhorar performance e otimizar o código.
- Documentar APIs para extensões/plugins.
- Entregável: Lançamento de uma versão beta com documentação completa.

---

## 5. Escopo Futuro
- Suporte para VR e AR.
- Simulações mais avançadas de dinâmica de fluidos.
- Integração com frameworks de Machine Learning para análise preditiva.

---
