import { useState, useRef, useEffect } from "react";
import { useAppStore } from "../stores/appStore";
import { chat, checkPremiumStatus } from "../services/api";
import { Message } from "../types";

export default function Chat() {
  const { providers, conversations, setConversations, currentConversationId, setCurrentConversationId } = useAppStore();
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState("");
  const [isStreaming, setIsStreaming] = useState(false);
  const [selectedModelId, setSelectedModelId] = useState("");
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (providers.length > 0 && !selectedModelId) {
      const first = providers.find(p => p.is_enabled) || providers[0];
      if (first) setSelectedModelId(first.id);
    }
  }, [providers, selectedModelId]);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  async function handleSend() {
    if (!input.trim() || isStreaming) return;
    const userMsg: Message = { id: Date.now().toString(), role: "user", content: input.trim(), created_at: new Date().toISOString() };
    setMessages(prev => [...prev, userMsg]);
    setInput("");
    setIsStreaming(true);

    try {
      const msgs = [...messages, userMsg].map(m => ({ role: m.role, content: m.content }));
      const response = await chat(msgs, selectedModelId);
      const aiMsg: Message = { id: (Date.now() + 1).toString(), role: "assistant", content: response, created_at: new Date().toISOString() };
      setMessages(prev => [...prev, aiMsg]);
    } catch (e: any) {
      const errMsg: Message = { id: (Date.now() + 1).toString(), role: "assistant", content: "[AI Error] " + (e?.message || e?.toString() || "未知错误"), created_at: new Date().toISOString() };
      setMessages(prev => [...prev, errMsg]);
    } finally {
      setIsStreaming(false);
    }
  }

  function handleKeyDown(e: React.KeyboardEvent) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  }

  return (
    <div className="chat-page">
      <div className="chat-header">
        <h2>AI 对话</h2>
        <select value={selectedModelId} onChange={e => setSelectedModelId(e.target.value)} className="model-select">
          {providers.map(p => (
            <option key={p.id} value={p.id}>
              {p.display_name}
            </option>
          ))}
        </select>
      </div>

      <div className="chat-messages">
        {messages.length === 0 && (
          <div className="chat-empty">
            <div className="empty-icon">💬</div>
            <p>开始与 AI 对话</p>
            <p className="empty-hint">选择模型后输入消息</p>
          </div>
        )}
        {messages.map(msg => (
          <div key={msg.id} className={`message ${msg.role}`}>
            <div className="message-avatar">{msg.role === "user" ? "" : "🤖"}</div>
            <div className="message-content">
              <div className="message-text">{msg.content}</div>
              <div className="message-time">{new Date(msg.created_at).toLocaleTimeString()}</div>
            </div>
          </div>
        ))}
        {isStreaming && (
          <div className="message assistant">
            <div className="message-avatar"></div>
            <div className="message-content">
              <div className="typing-indicator">
                <span></span><span></span><span></span>
              </div>
            </div>
          </div>
        )}
        <div ref={messagesEndRef} />
      </div>

      <div className="chat-input-area">
        <textarea
          value={input}
          onChange={e => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="输入消息... (Enter 发送，Shift+Enter 换行)"
          rows={3}
          disabled={isStreaming}
        />
        <button onClick={handleSend} disabled={isStreaming || !input.trim()}>
          {isStreaming ? "发送中..." : "发送"}
        </button>
      </div>
    </div>
  );
}
