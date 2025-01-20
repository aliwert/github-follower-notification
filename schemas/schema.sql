-- Notification Events Table
CREATE TABLE notification_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    event_type VARCHAR(50) NOT NULL,
    sender_login VARCHAR(100) NOT NULL,
    sender_avatar_url TEXT,
    sender_html_url TEXT,
    processed BOOLEAN DEFAULT FALSE
);

-- Service Notifications Table
CREATE TABLE service_notifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id INTEGER NOT NULL,
    service_type VARCHAR(20) NOT NULL, -- 'whatsapp', 'telegram', 'discord', 'slack', 'email'
    status VARCHAR(20) NOT NULL, -- 'pending', 'sent', 'failed'
    sent_at TIMESTAMP,
    error_message TEXT,
    FOREIGN KEY (event_id) REFERENCES notification_events(id)
);

-- Service Configurations
CREATE TABLE service_configs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    service_type VARCHAR(20) UNIQUE NOT NULL,
    enabled BOOLEAN DEFAULT TRUE,
    config_json TEXT NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX idx_notification_events_created_at ON notification_events(created_at);
CREATE INDEX idx_notification_events_processed ON notification_events(processed);
CREATE INDEX idx_service_notifications_event_id ON service_notifications(event_id);
CREATE INDEX idx_service_notifications_status ON service_notifications(status);

-- Create views
CREATE VIEW v_pending_notifications AS
SELECT 
    ne.id as event_id,
    ne.event_type,
    ne.sender_login,
    ne.created_at,
    COUNT(sn.id) as pending_services
FROM notification_events ne
LEFT JOIN service_notifications sn ON ne.id = sn.event_id
WHERE ne.processed = FALSE
GROUP BY ne.id;

-- Insert default service configs
INSERT INTO service_configs (service_type, config_json) VALUES
('whatsapp', '{"enabled": false}'),
('telegram', '{"enabled": false}'),
('discord',  '{"enabled": false}'),
('slack',    '{"enabled": false}'),
('email',    '{"enabled": false}');