type Time = {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
};

export type Event = {
  name: string;
  uuid: string;
  start: Time;
  end: Time;
};

interface EventsObject {
  [key: string]: Array<Event>;
}

export class EventDisplayManager {
  private static instance: EventDisplayManager;
  private events: EventsObject;

  private constructor() {
    this.events = {};
  }

  public static getInstance(): EventDisplayManager {
    if (!EventDisplayManager.instance) {
      EventDisplayManager.instance = new EventDisplayManager();
    }
    return EventDisplayManager.instance;
  }

  public addEvent(calendar: string, event: Event): void {
    const eventKey = `${calendar}-${event.start.year}-${event.start.month}-${event.start.day}`;
    if (!this.events[eventKey]) {
      this.events[eventKey] = [];
    }
    this.events[eventKey].push(event);
  }

  public getEvents(): EventsObject {
    return this.events;
  }

  public clearEvents() {
    this.events = {};
  }
}

export class EventManager {
  private static instance: EventManager;
  
  public static getInstance(): EventManager {
    if(!EventManager.instance) {
      EventManager.instance = new EventManager();
    }
    return EventManager.instance;
  }

  public postEvent() {
    console.log("Post event");
  }
}
