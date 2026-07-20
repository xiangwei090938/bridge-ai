import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface NewsItem {
  id: string;
  title: string;
  summary: string;
  source: string;
  category: string;
  tags: string;
  url: string;
  published_at: string;
  fetched_at: string;
}

const CATEGORIES = ["全部", "模型发布", "工具更新", "开源动态", "行业分析", "技术应用", "行业动态"];

// 默认 Mock 数据（后端不可用时降级）
const MOCK_NEWS: NewsItem[] = [
  {
    id: "1",
    title: "DeepSeek V3 正式发布，性能媲美 GPT-4",
    summary: "DeepSeek V3 模型正式开源，在多个基准测试中表现优异，支持本地部署，推理成本大幅降低。",
    source: "机器之心",
    category: "模型发布",
    tags: "DeepSeek,开源,模型",
    url: "https://www.jiqizhixin.com/",
    published_at: "2026-07-20 10:00:00",
    fetched_at: "2026-07-20 10:05:00",
  },
  {
    id: "2",
    title: "通义千问 Qwen3 发布，支持 100+ 语言",
    summary: "阿里发布通义千问 Qwen3，支持超过 100 种语言，多语言能力领先，适合全球化应用场景。",
    source: "量子位",
    category: "模型发布",
    tags: "Qwen,阿里,多语言",
    url: "https://www.qbitai.com/",
    published_at: "2026-07-19 08:00:00",
    fetched_at: "2026-07-19 08:30:00",
  },
  {
    id: "3",
    title: "Cursor IDE 更新：支持 AI 自动重构代码",
    summary: "Cursor IDE 最新版本支持 AI 驱动的代码重构，可自动优化代码结构，提升开发效率。",
    source: "36 氪",
    category: "工具更新",
    tags: "Cursor,IDE,代码重构",
    url: "https://36kr.com/",
    published_at: "2026-07-18 14:00:00",
    fetched_at: "2026-07-18 14:20:00",
  },
  {
    id: "4",
    title: "2026 年 AI 行业趋势报告：大模型进入实用阶段",
    summary: "最新报告显示，大语言模型已从实验阶段进入企业实用阶段，超过 60% 的企业已部署 AI 应用。",
    source: "InfoQ 中文",
    category: "行业分析",
    tags: "趋势,报告,企业应用",
    url: "https://www.infoq.cn/",
    published_at: "2026-07-17 16:00:00",
    fetched_at: "2026-07-17 16:15:00",
  },
];

export default function News() {
  const [news, setNews] = useState<NewsItem[]>(MOCK_NEWS);
  const [selectedCategory, setSelectedCategory] = useState("全部");
  const [searchQuery, setSearchQuery] = useState("");
  const [loading, setLoading] = useState(false);
  const [lastFetch, setLastFetch] = useState<string>("");
  const [refreshMsg, setRefreshMsg] = useState("");
  const [autoFetchInfo, setAutoFetchInfo] = useState("");

  // 从后端加载资讯
  const loadNews = async (category?: string) => {
    setLoading(true);
    try {
      const res: any = await invoke("get_news", {
        category: category === "全部" ? null : category,
        limit: 50,
      });
      if (res.success && res.items.length > 0) {
        setNews(res.items);
        setLastFetch(res.last_fetch || "");
      }
    } catch (e) {
      console.warn("后端资讯不可用，使用模拟数据");
    } finally {
      setLoading(false);
    }
  };

  // 手动刷新资讯
  const handleRefresh = async () => {
    setLoading(true);
    setRefreshMsg("正在采集最新资讯...");
    try {
      const res: any = await invoke("refresh_news");
      if (res.success) {
        setRefreshMsg(res.message);
        await loadNews(selectedCategory === "全部" ? undefined : selectedCategory);
      } else {
        setRefreshMsg(res.message || "采集失败");
      }
    } catch (e: any) {
      setRefreshMsg(`采集失败：${e}`);
    } finally {
      setLoading(false);
      setTimeout(() => setRefreshMsg(""), 3000);
    }
  };

  // 清除英文文章
  const handleClearEnglish = async () => {
    setLoading(true);
    setRefreshMsg("正在清除英文文章...");
    try {
      const res: any = await invoke("clear_non_chinese_news");
      if (res.success) {
        setRefreshMsg(res.message);
        await loadNews(selectedCategory === "全部" ? undefined : selectedCategory);
      } else {
        setRefreshMsg(res.message || "清除失败");
      }
    } catch (e: any) {
      setRefreshMsg(`清除失败：${e}`);
    } finally {
      setLoading(false);
      setTimeout(() => setRefreshMsg(""), 3000);
    }
  };

  // 打开文章链接（使用 Tauri shell 打开系统浏览器）
  const handleOpenUrl = async (url: string) => {
    try {
      await invoke("open_url", { url });
    } catch (e) {
      // 降级：浏览器模式下直接用 window.open
      window.open(url, "_blank");
    }
  };

  useEffect(() => {
    loadNews();
    setAutoFetchInfo("⏰ 每天 8:00 自动采集最新资讯");
  }, []);

  const filteredNews = news.filter((item) => {
    const matchCategory =
      selectedCategory === "全部" || item.category === selectedCategory;
    const matchSearch =
      searchQuery === "" ||
      item.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      item.summary.toLowerCase().includes(searchQuery.toLowerCase());
    return matchCategory && matchSearch;
  });

  return (
    <div className="news-page">
      {/* Header */}
      <div className="news-header">
        <div>
          <h1>AI 资讯</h1>
          <p className="news-subtitle">每日 AI 简报 · 行业动态聚合</p>
          {lastFetch && (
            <p className="news-last-fetch">最近更新：{lastFetch}</p>
          )}
          <p className="news-auto-info">{autoFetchInfo}</p>
        </div>
        <div className="news-actions">
          <button
            className="news-clear-btn"
            onClick={handleClearEnglish}
            disabled={loading}
          >
            清除英文
          </button>
          <button
            className="news-refresh-btn"
            onClick={handleRefresh}
            disabled={loading}
          >
            {loading ? "采集中..." : "刷新资讯"}
          </button>
        </div>
      </div>

      {refreshMsg && <div className="news-refresh-msg">{refreshMsg}</div>}

      {/* Filters */}
      <div className="news-filters">
        <div className="category-tabs">
          {CATEGORIES.map((cat) => (
            <button
              key={cat}
              className={
                "cat-tab" + (selectedCategory === cat ? " active" : "")
              }
              onClick={() => {
                setSelectedCategory(cat);
                loadNews(cat === "全部" ? undefined : cat);
              }}
            >
              {cat}
            </button>
          ))}
        </div>
        <div className="search-box">
          <svg
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
          >
            <circle cx="11" cy="11" r="8" />
            <path d="m21 21-4.35-4.35" />
          </svg>
          <input
            type="text"
            placeholder="搜索资讯..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
        </div>
      </div>

      {/* News List */}
      <div className="news-list">
        {filteredNews.length === 0 ? (
          <div className="news-empty">
            <svg
              width="48"
              height="48"
              viewBox="0 0 24 24"
              fill="none"
              stroke="var(--text-muted)"
              strokeWidth="1.5"
              opacity="0.3"
            >
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
              <polyline points="14 2 14 8 20 8" />
            </svg>
            <span>暂无相关资讯</span>
          </div>
        ) : (
          filteredNews.map((item) => (
            <div key={item.id} className="news-card">
              <div className="news-card-header">
                <span className="news-category">{item.category}</span>
                <span className="news-date">{item.published_at}</span>
              </div>
              <h3 className="news-title">{item.title}</h3>
              <p className="news-summary">{item.summary}</p>
              {item.tags && (
                <div className="news-tags">
                  {item.tags.split(",").map((tag, idx) => (
                    <span key={idx} className="news-tag">
                      {tag}
                    </span>
                  ))}
                </div>
              )}
              <div className="news-footer">
                <span className="news-source">{item.source}</span>
                <button
                  className="news-link-btn"
                  onClick={() => handleOpenUrl(item.url)}
                >
                  阅读原文
                  <svg
                    width="12"
                    height="12"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    strokeWidth="2"
                  >
                    <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
                    <polyline points="15 3 21 3 21 9" />
                    <line x1="10" y1="14" x2="21" y2="3" />
                  </svg>
                </button>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
