use super::StatsReportType;

#[derive(Debug, Default)]
pub struct StatsCollector {
    pub(crate) reports: Vec<StatsReportType>,
}

impl StatsCollector {
    pub(crate) fn new() -> Self {
        StatsCollector {
            ..Default::default()
        }
    }

    pub(crate) fn append(&mut self, stats: &mut Vec<StatsReportType>) {
        // TODO increase capacity when needed
        self.reports.append(stats);
    }

    pub(crate) fn push(&mut self, stats: StatsReportType) {
        // TODO increase capacity when needed
        self.reports.push(stats);
    }
}
