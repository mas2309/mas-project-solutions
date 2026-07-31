use std::sync::Arc;
use crate::application::repositories::gasto_recurrente_repository::IGastoRecurrenteRepository;
use crate::application::repositories::gasto_repository::IGastoRepository;
use crate::domain::entities::{GastoRecurrente, TipoGastoRecurrente, Gasto};
use crate::application::dto::{CreateGastoRecurrenteDto, CreateGastoDto, GenerarGastosDto};
use anyhow::{Result, anyhow};
use chrono::Datelike;

pub struct GastoRecurrenteService {
    repository: Arc<dyn IGastoRecurrenteRepository>,
    gasto_repository: Arc<dyn IGastoRepository>,
}

impl GastoRecurrenteService {
    pub fn new(
        repository: Arc<dyn IGastoRecurrenteRepository>,
        gasto_repository: Arc<dyn IGastoRepository>,
    ) -> Self {
        Self { repository, gasto_repository }
    }

    pub async fn crear(&self, usuario_id: i64, dto: CreateGastoRecurrenteDto) -> Result<GastoRecurrente> {
        self.repository.create(usuario_id, dto).await
    }

    pub async fn listar_todos(&self, usuario_id: i64) -> Result<Vec<GastoRecurrente>> {
        self.repository.list_all(usuario_id).await
    }

    pub async fn obtener(&self, usuario_id: i64, id: i32) -> Result<GastoRecurrente> {
        self.repository.find_by_id(usuario_id, id).await?
            .ok_or_else(|| anyhow!("Gasto recurrente no encontrado"))
    }

    pub async fn editar(&self, usuario_id: i64, id: i32, dto: CreateGastoRecurrenteDto) -> Result<GastoRecurrente> {
        self.repository.update(usuario_id, id, dto).await?
            .ok_or_else(|| anyhow!("Gasto recurrente no encontrado"))
    }

    pub async fn toggle_activo(&self, usuario_id: i64, id: i32) -> Result<GastoRecurrente> {
        self.repository.toggle_activo(usuario_id, id).await?
            .ok_or_else(|| anyhow!("Gasto recurrente no encontrado"))
    }

    pub async fn eliminar(&self, usuario_id: i64, id: i32) -> Result<GastoRecurrente> {
        self.repository.delete(usuario_id, id).await?
            .ok_or_else(|| anyhow!("Gasto recurrente no encontrado"))
    }

    /// Genera automáticamente los gastos FIJOS cuyo día de facturación ya pasó en el mes actual.
    /// Se llama automáticamente al entrar a la sección de gastos o gastos recurrentes.
    /// Solo genera los que NO se hayan generado ya para ese mes.
    pub async fn auto_generar_fijos(&self, usuario_id: i64) -> Result<Vec<Gasto>> {
        let now = chrono::Local::now();
        let dia_actual = now.day() as i32;
        let mes = now.month();
        let anio = now.year();

        // Obtener plantillas activas de tipo Fijo
        let plantillas = self.repository.list_activos(usuario_id).await?;
        let fijos: Vec<&GastoRecurrente> = plantillas.iter()
            .filter(|p| p.tipo == TipoGastoRecurrente::Fijo && p.dia_facturacion <= dia_actual)
            .collect();

        if fijos.is_empty() {
            return Ok(vec![]);
        }

        // Ver cuáles ya se generaron este mes
        let ya_generados = self.repository.ya_generados_en_mes(usuario_id, anio, mes).await?;

        // Generar solo los pendientes
        let mut gastos_generados = Vec::new();

        for plantilla in fijos {
            if ya_generados.contains(&plantilla.id) {
                continue;
            }

            // La fecha del gasto es el día de facturación del mes actual
            let dia = plantilla.dia_facturacion.min(28); // Proteger contra meses cortos
            let fecha = format!("{}-{:02}-{:02}", anio, mes, dia);

            let gasto_dto = CreateGastoDto {
                descripcion: plantilla.descripcion.clone(),
                monto: plantilla.monto_referencia,
                categoria: plantilla.categoria.clone(),
                responsable: plantilla.responsable.clone(),
                fecha,
            };

            let gasto = self.gasto_repository.create_from_recurrente(usuario_id, gasto_dto, plantilla.id).await?;
            gastos_generados.push(gasto);
        }

        if !gastos_generados.is_empty() {
            println!("📌 Auto-generados {} gastos fijos del mes", gastos_generados.len());
        }

        Ok(gastos_generados)
    }

    /// Genera manualmente los gastos FIJO VARIABLE del mes indicado.
    /// Se ejecuta cuando el usuario presiona el botón "Generar".
    pub async fn generar_variables_del_mes(&self, usuario_id: i64, dto: GenerarGastosDto) -> Result<Vec<Gasto>> {
        // Obtener plantillas activas de tipo FijoVariable
        let plantillas = self.repository.list_activos(usuario_id).await?;
        let variables: Vec<&GastoRecurrente> = plantillas.iter()
            .filter(|p| p.tipo == TipoGastoRecurrente::FijoVariable)
            .collect();

        if variables.is_empty() {
            return Ok(vec![]);
        }

        // Ver cuáles ya se generaron este mes
        let ya_generados = self.repository.ya_generados_en_mes(usuario_id, dto.anio, dto.mes).await?;

        // Generar solo los pendientes
        let mut gastos_generados = Vec::new();

        for plantilla in variables {
            if ya_generados.contains(&plantilla.id) {
                continue;
            }

            let dia = plantilla.dia_facturacion.min(28);
            let fecha = format!("{}-{:02}-{:02}", dto.anio, dto.mes, dia);

            let gasto_dto = CreateGastoDto {
                descripcion: plantilla.descripcion.clone(),
                monto: plantilla.monto_referencia,
                categoria: plantilla.categoria.clone(),
                responsable: plantilla.responsable.clone(),
                fecha,
            };

            let gasto = self.gasto_repository.create_from_recurrente(usuario_id, gasto_dto, plantilla.id).await?;
            gastos_generados.push(gasto);
        }

        Ok(gastos_generados)
    }

    /// Verifica cuántos gastos fijo-variable faltan por generar en el mes
    pub async fn variables_pendientes_por_generar(&self, usuario_id: i64, anio: i32, mes: u32) -> Result<u32> {
        let plantillas = self.repository.list_activos(usuario_id).await?;
        let ya_generados = self.repository.ya_generados_en_mes(usuario_id, anio, mes).await?;
        let pendientes = plantillas.iter()
            .filter(|p| p.tipo == TipoGastoRecurrente::FijoVariable && !ya_generados.contains(&p.id))
            .count() as u32;
        Ok(pendientes)
    }
}
