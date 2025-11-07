# Hodei Audit API - Quick Reference

## Base URL
```
Development: http://localhost:3000/api
Production: https://api.hodei-audit.com/v1
```

## Authentication

### Bearer Token (JWT)
```bash
Authorization: Bearer your-jwt-token
```

### API Key
```bash
x-api-key: your-api-key
```

## Quick Start

### 1. Get Events
```bash
curl -H "Authorization: Bearer token_admin" \
     "http://localhost:3000/api/events?page=1&pageSize=50"
```

### 2. Run Analytics Query
```bash
curl -X POST \
     -H "Authorization: Bearer token_admin" \
     -H "Content-Type: application/json" \
     -d '{
       "query": {
         "tenantId": "tenant-1",
         "aggregations": [{"function": "count", "field": "id"}]
       }
     }' \
     "http://localhost:3000/api/analytics/query"
```

### 3. Generate Compliance Report
```bash
curl -X POST \
     -H "Authorization: Bearer token_admin" \
     -H "Content-Type: application/json" \
     -d '{
       "name": "Monthly SOC2 Report",
       "type": "SOC2",
       "format": "pdf",
       "timeRange": {
         "start": "2025-11-01T00:00:00Z",
         "end": "2025-11-30T23:59:59Z"
       }
     }' \
     "http://localhost:3000/api/compliance/reports"
```

### 4. Subscribe to Real-time Events (SSE)
```javascript
const eventSource = new EventSource(
  "http://localhost:3000/api/sse/stream?tenantId=tenant-1&eventTypes=new_event,user_activity"
);

eventSource.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log("New event:", data);
};
```

## API Endpoints

### Events
- `GET /api/events` - Query events
- `GET /api/events/{id}` - Get event by ID
- `POST /api/events/export` - Export events

### Analytics
- `POST /api/analytics/query` - Run analytics query
- `GET /api/analytics/saved` - Get saved queries
- `POST /api/analytics/saved` - Save query

### Compliance
- `GET /api/compliance/reports` - Get reports
- `POST /api/compliance/reports` - Generate report
- `GET /api/compliance/keys` - Get cryptographic keys
- `POST /api/compliance/keys/{id}/rotate` - Rotate key

### Real-time
- `GET /api/sse/stream` - SSE stream for real-time updates

### Documentation
- `GET /api/docs?format=json` - OpenAPI spec
- `GET /api/docs?format=markdown` - Markdown docs

## Response Format

### Success Response
```json
{
  "success": true,
  "data": {...},
  "meta": {
    "requestId": "req_1234567890",
    "timestamp": "2025-11-07T10:30:00Z"
  }
}
```

### Error Response
```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid parameters",
    "details": {...},
    "requestId": "req_1234567890",
    "timestamp": "2025-11-07T10:30:00Z"
  }
}
```

## Status Codes
- `200` - Success
- `201` - Created
- `400` - Bad Request
- `401` - Unauthorized
- `403` - Forbidden
- `404` - Not Found
- `429` - Rate Limited
- `500` - Internal Error

## Rate Limits
- General API: 100 requests/minute
- Event queries: 60 requests/minute
- Analytics: 30 requests/minute
- Compliance: 20 requests/minute
- Export: 5 requests/minute

## Testing with Mock Data

Set environment variable for mock mode:
```bash
export NEXT_PUBLIC_USE_MOCK_API=true
```

This enables mock APIs for development without backend services.

## Common Use Cases

### Filter Events by Date Range
```bash
GET /api/events?startDate=2025-11-01T00:00:00Z&endDate=2025-11-07T23:59:59Z
```

### Paginate Results
```bash
GET /api/events?page=2&pageSize=20
```

### Sort Events
```bash
GET /api/events?sortBy=timestamp&sortOrder=desc
```

### Export Events
```bash
POST /api/events/export
{
  "tenantId": "tenant-1",
  "format": "csv"
}
```

## WebSocket for Real-time

Connect to WebSocket for instant event notifications:
```javascript
const ws = new WebSocket("ws://localhost:8080/ws");
ws.onopen = () => {
  ws.send(JSON.stringify({
    type: "subscribe",
    eventType: "new_event",
    filters: { tenantId: "tenant-1" }
  }));
};
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  // Handle new event
};
```

## Support
- Email: support@hodei-audit.com
- Documentation: https://docs.hodei-audit.com
- API Status: https://status.hodei-audit.com
