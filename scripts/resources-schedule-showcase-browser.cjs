// Maintained entry point; exercise the current independently compiled documentation sections.
process.env.SAPID_SCHEDULE_PAGES='resources-schedule';
process.env.SAPID_SCHEDULE_THEMES='blue';
process.env.SAPID_SCHEDULE_SCHEMES='light';
process.env.SAPID_SCHEDULE_WIDTHS='1280';
process.env.SAPID_SCHEDULE_REPORT='resources-schedule-showcase-browser.json';
require('./showcase-remediation-schedule.cjs');
