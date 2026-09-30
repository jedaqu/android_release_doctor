# M0.6 → M0.7 — Secuencia de transición

Fecha de inicio: 2026-09-30

Este documento fija la secuencia de trabajo posterior a M0.6. Las etapas no se consideran finalizadas por intención: cada una pasa a **FINALIZADA** únicamente cuando cumple sus criterios de cierre y existe evidencia registrada en el repositorio.

## Disciplina obligatoria

Cada etapa técnica seguirá, cuando corresponda, esta secuencia:

**ledger review → audit → scoped changes → second audit → Actions → follow-up → individual correction → new validation → checkpoint**

No se inicia una etapa posterior mientras la anterior tenga hallazgos de cierre pendientes.

## Etapa 1 — Cierre formal de M0.6

**Estado: FINALIZADA**

Objetivo: dejar M0.6 formalmente cerrado como línea de desarrollo, sin mezclar todavía nuevas capacidades de M0.7.

Verificaciones de cierre:

- ledger incremental revisado;
- Blocks 1–5 de M0.6 con sus auditorías y checkpoints;
- evaluación global de M0.6 cerrada;
- fronteras de capacidad documentadas;
- README coherente con la cronología real;
- validación CI final registrada;
- PRs de la línea M0.6 permanecen sin merge conforme a la disciplina establecida;
- ausencia de hallazgos de producción pendientes.

Evidencia de cierre:

- `docs/CLOSURE-M0.6.md`;
- `docs/SECOND-AUDIT-M0.6-CLOSURE.md`;
- Actions run #260 / 36754421382: Build PASS, Test PASS, Format PASS, Clippy PASS.

Criterio cumplido: no se identificó ninguna corrección pendiente en el contenido de cierre. El checkpoint de esta etapa se registra en la actualización de estado que acompaña a esta evidencia.

## Etapa 2 — Auditoría de transición M0.6 → M0.7

**Estado: FINALIZADA**

Objetivo: determinar, mediante evidencia y no por anticipación, qué trabajo tiene sentido abrir como M0.7.

La auditoría separará cada capacidad en:

1. **Implementado y verificado**
2. **Implementado pero deliberadamente limitado**
3. **No implementado**

Se revisarán, entre otros, estos límites ya identificados:

- cobertura criptográfica restante de algoritmos y tamaños de clave Android;
- v3.1;
- v3.2/PQC;
- verificación criptográfica de AAB;
- evaluación Gradle/variantes;
- automatización de políticas actuales de Google Play;
- salida HTML/SARIF;
- clasificación de riesgo de permisos.

La auditoría no asumirá que todos esos puntos pertenecen a M0.7. Se determinará su valor, dependencia, verificabilidad y alcance antes de asignarlos.

Cierre requerido:
- auditoría de transición aprobada;
- segunda auditoría aprobada;
- límites y dependencias documentados;
- CI de la documentación de transición validado;
- checkpoint formal registrado;
- ninguna capacidad promovida a M0.7 sin evidencia suficiente.

Evidencia de cierre:

- `docs/AUDIT-M0.6-TRANSITION.md` — auditoría de transición aprobada;
- `docs/SECOND-AUDIT-M0.6-TRANSITION.md` — segunda auditoría aprobada;
- Actions run #270 / 36762113690 — documentación de transición validada;
- Actions run #272 / 36762143731 — roadmap validado con Build/Test/Format/Clippy PASS;
- checkpoint formal: `docs/CHECKPOINT-M0.6-TRANSITION-FINAL.md`.

## Etapa 3 — Definición de M0.7

**Estado: FINALIZADA**

Objetivo: convertir los resultados de la auditoría de transición en un alcance M0.7 pequeño, verificable y trazable.

La definición deberá establecer:

- objetivo de M0.7;
- capacidades incluidas;
- capacidades explícitamente fuera de alcance;
- bloques propuestos y dependencias;
- criterios de aceptación;
- estrategia de fixtures y regresiones;
- requisitos de CI para ramas apiladas;
- checkpoint de baseline de M0.7.

Cierre requerido:
- alcance M0.7 documentado;
- orden de bloques definido;
- criterios de aceptación definidos;
- capacidades explícitamente fuera de alcance;
- estrategia de fixtures y regresiones definida;
- estrategia de CI para ramas apiladas definida;
- segunda auditoría de la definición aprobada;
- CI de la definición validado;
- baseline/checkpoint de inicio registrado.

Evidencia de cierre:
- `docs/M0.7-DEFINITION.md` — alcance y método definidos y corregidos tras la segunda auditoría;
- `docs/SECOND-AUDIT-M0.7-DEFINITION.md` — segunda auditoría PASS;
- Actions run #289 / 36766123681 — push validation SUCCESS;
- Actions run #290 / 36766132670 — stacked PR-event validation SUCCESS;
- `docs/CHECKPOINT-M0.7-DEFINITION-FINAL.md` — checkpoint formal de cierre;
- M0.6 baseline SHA: `5979845936083475829f9b9cd797fb4969867362`.

## Regla de finalización

Una etapa solo se marca como **FINALIZADA** después de comprobar su evidencia y registrar su cierre.

La secuencia no se salta:

**Etapa 1 → Etapa 2 → Etapa 3**

No se iniciará implementación de M0.7 antes de completar la Etapa 3.
