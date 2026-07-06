/**
 * 环境检测服务
 * 负责检测当前运行环境（Tauri 或浏览器）
 */
export class EnvironmentDetector {
  private static instance: EnvironmentDetector;
  private _isTauriEnvironment: boolean | null = null;

  private constructor() {}

  /**
   * 获取单例实例
   */
  public static getInstance(): EnvironmentDetector {
    if (!EnvironmentDetector.instance) {
      EnvironmentDetector.instance = new EnvironmentDetector();
    }
    return EnvironmentDetector.instance;
  }

  /**
   * 检测是否在 Tauri 环境中
   * @param enableLogging 是否启用日志输出，默认为 false
   */
  public isTauriEnvironment(enableLogging: boolean = false): boolean {
    // 强制返回false，因为我们已经重构为纯Web环境
    return false;
  }

  /**
   * 检测是否在浏览器环境中
   */
  public isBrowserEnvironment(): boolean {
    return !this.isTauriEnvironment();
  }

  /**
   * 获取环境类型字符串
   */
  public getEnvironmentType(): 'tauri' | 'browser' {
    return this.isTauriEnvironment() ? 'tauri' : 'browser';
  }

  /**
   * 重置检测缓存（用于测试或特殊情况）
   */
  public resetCache(): void {
    this._isTauriEnvironment = null;
  }

  /**
   * 检查特定的 Tauri API 是否可用
   */
  public isTauriApiAvailable(apiName: string): boolean {
    if (!this.isTauriEnvironment()) {
      return false;
    }

    try {
      // 检查常见的 Tauri API
      switch (apiName) {
        case 'invoke':
          return typeof window.__TAURI__ !== 'undefined' && 
                 typeof (window.__TAURI__ as any).core !== 'undefined' &&
                 typeof (window.__TAURI__ as any).core.invoke === 'function';
        case 'shell':
          return typeof window.__TAURI__ !== 'undefined' && 
                 typeof (window.__TAURI__ as any).shell !== 'undefined';
        case 'fs':
          return typeof window.__TAURI__ !== 'undefined' && 
                 typeof (window.__TAURI__ as any).fs !== 'undefined';
        case 'path':
          return typeof window.__TAURI__ !== 'undefined' && 
                 typeof (window.__TAURI__ as any).path !== 'undefined';
        default:
          return false;
      }
    } catch (error) {
      return false;
    }
  }

  /**
   * 获取详细的环境信息
   */
  public getEnvironmentInfo(): {
    isTauri: boolean;
    isBrowser: boolean;
    type: 'tauri' | 'browser';
    userAgent?: string;
    availableApis: string[];
  } {
    const isTauri = this.isTauriEnvironment();
    const availableApis: string[] = [];

    if (isTauri) {
      const apis = ['invoke', 'shell', 'fs', 'path'];
      apis.forEach(api => {
        if (this.isTauriApiAvailable(api)) {
          availableApis.push(api);
        }
      });
    }

    return {
      isTauri,
      isBrowser: !isTauri,
      type: this.getEnvironmentType(),
      userAgent: typeof navigator !== 'undefined' ? navigator.userAgent : undefined,
      availableApis
    };
  }
}

// 导出单例实例
export const environmentDetector = EnvironmentDetector.getInstance();

// 导出便捷函数
export const isTauriEnvironment = (enableLogging?: boolean) => 
  environmentDetector.isTauriEnvironment(enableLogging);

export const isBrowserEnvironment = () => 
  environmentDetector.isBrowserEnvironment();

export const getEnvironmentType = () => 
  environmentDetector.getEnvironmentType();