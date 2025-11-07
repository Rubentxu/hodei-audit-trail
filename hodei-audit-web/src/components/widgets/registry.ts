import { WidgetRegistry } from './types';

// Import widget components
import { EventCountWidget } from './event-count-widget';
import { CriticalEventsWidget } from './critical-events-widget';
import { ComplianceScoreWidget } from './compliance-score-widget';
import { TimeSeriesWidget } from './time-series-widget';
import { TopUsersWidget } from './top-users-widget';
import { ErrorRateWidget } from './error-rate-widget';

// Register all widgets
export const widgetRegistry: WidgetRegistry = {
  'event-count': {
    component: EventCountWidget,
    defaultSize: { w: 1, h: 1 },
    displayName: 'Event Count',
    description: 'Display total number of events with trend',
  },
  'critical-events': {
    component: CriticalEventsWidget,
    defaultSize: { w: 1, h: 1 },
    displayName: 'Critical Events',
    description: 'Show critical events and warnings',
  },
  'compliance-score': {
    component: ComplianceScoreWidget,
    defaultSize: { w: 1, h: 1 },
    displayName: 'Compliance Score',
    description: 'Display compliance score with circular progress',
  },
  'time-series': {
    component: TimeSeriesWidget,
    defaultSize: { w: 2, h: 1 },
    displayName: 'Time Series',
    description: 'Show events over time as a line chart',
  },
  'top-users': {
    component: TopUsersWidget,
    defaultSize: { w: 1, h: 1 },
    displayName: 'Top Users',
    description: 'Display most active users',
  },
  'error-rate': {
    component: ErrorRateWidget,
    defaultSize: { w: 1, h: 1 },
    displayName: 'Error Rate',
    description: 'Show error rate and failed requests',
  },
};

export function getWidgetType(type: string) {
  return widgetRegistry[type];
}

export function getAllWidgetTypes() {
  return Object.entries(widgetRegistry).map(([key, value]) => ({
    type: key,
    ...value,
  }));
}

export function getAvailableWidgetTypes() {
  return Object.entries(widgetRegistry).map(([type, config]) => ({
    type,
    displayName: config.displayName,
    description: config.description,
  }));
}
