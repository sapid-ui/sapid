// Maintained entry point; exercise the current independently compiled documentation sections.
process.env.SAPID_SCHEDULE_PAGES='day-view,week-view';
process.env.SAPID_SCHEDULE_THEMES='blue';
process.env.SAPID_SCHEDULE_SCHEMES='light';
process.env.SAPID_SCHEDULE_WIDTHS='1280';
process.env.SAPID_SCHEDULE_REPORT='schedule-timelines-showcase-browser.json';
require('./showcase-remediation-schedule.cjs');
