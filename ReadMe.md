# Rusty-clock - The clock you don't need

![Rusty Clock GUI](/images/Rusty-clock-gui.jpg "Start Tracking your day!").
## How to run the rusty-clock

#### To enter in cli mode
```
cargo run
```
#### To enter GUI mode
```
cargo run -- --gui
```
## Objetivos:
- [x] Ter um cli
- [x] Receber um comando para dar start de timer especifico
- [x] Tipos de timers -> Study, Work, Fun, Coffee
- [x] Apenas um timer pode estar ativo
- [x] Dá para mudar de timer sem ter de terminar o outro primeiro
- [ ] Gravar valores por dia, semana, mes, ano e fazer estatisticas
- [ ] Avisar a cada x minutos que timer está a decorrer
- [ ] Notificar de y em y a perguntar o que se está a fazer
- [x] Ter num nice gui :) probably tui-rs oops, icedrs afinal

## Ordem Tarefas:
- [x] cli 
- [x] types of timers
- [x] start timer commands
- [x] stop timer commands
- [x] save data
- [ ] notifiyer
- [x] gui
- [ ] clean up
- [ ] gui mostrar dados atuais de cada timer
- [ ] mais dados na db