# ADR Scope Disclaimer

ADR (Agent-Oriented Declarative Runtime) ist eine
Safety-Orchestrierungsruntime für KI-Agenten auf Betriebssystem-Ebene.

## ADR ersetzt NICHT:

- Deterministische Echtzeit-Sicherheitsschleifen (SIL-4 nach IEC 61508)
- Hardware-Interlocks und physische Sicherheitssysteme
- Fail-Safe-Schaltkreise in kritischer Infrastruktur
- Zertifizierte RTOS-Systeme in Avionik oder Medizinprodukten

## ADR arbeitet NEBEN diesen Systemen, nicht statt ihnen.

Ein ADR-Agent kann ein Notfallprotokoll einleiten –
aber der physische Notschalter muss unabhängig davon funktionieren.

---
*Gemeinsam vereinbart von Claude (Anthropic) und ChatGPT (OpenAI).*
