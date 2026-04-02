
Constants
ADD_ORDER_EVENT	Indicates that an order has been added to the order book.
BUY_EVENT	Indicates a buy, with specific meaning that can vary depending on the situation. For example, when combined with a depth event, it means a bid-side event, while when combined with a trade event, it means that the trade initiator is a buyer.
CANCEL_ORDER_EVENT	Indicates that an order in the order book has been canceled.
DEPTH_BBO_EVENT	Indicates that the best bid and best ask update event is received.
DEPTH_CLEAR_EVENT	Indicates that the market depth is cleared.
DEPTH_EVENT	Indicates that the market depth is changed.
DEPTH_SNAPSHOT_EVENT	Indicates that the market depth snapshot is received.
EXCH_ADD_ORDER_EVENT	Represents a combination of EXCH_EVENT and ADD_ORDER_EVENT.
EXCH_ASK_ADD_ORDER_EVENT	Represents a combination of SELL_EVENT and EXCH_ADD_ORDER_EVENT.
EXCH_ASK_DEPTH_BBO_EVENT	Represents a combination of DEPTH_BBO_EVENT, SELL_EVENT, and EXCH_EVENT.
EXCH_ASK_DEPTH_CLEAR_EVENT	Represents a combination of DEPTH_CLEAR_EVENT, SELL_EVENT, and EXCH_EVENT.
EXCH_ASK_DEPTH_EVENT	Represents a combination of DEPTH_EVENT, SELL_EVENT, and EXCH_EVENT.
EXCH_ASK_DEPTH_SNAPSHOT_EVENT	Represents a combination of DEPTH_SNAPSHOT_EVENT, SELL_EVENT, and EXCH_EVENT.
EXCH_BID_ADD_ORDER_EVENT	Represents a combination of BUY_EVENT and EXCH_ADD_ORDER_EVENT.
EXCH_BID_DEPTH_BBO_EVENT	Represents a combination of DEPTH_BBO_EVENT, BUY_EVENT, and EXCH_EVENT.
EXCH_BID_DEPTH_CLEAR_EVENT	Represents a combination of DEPTH_CLEAR_EVENT, BUY_EVENT, and EXCH_EVENT.
EXCH_BID_DEPTH_EVENT	Represents a combination of DEPTH_EVENT, BUY_EVENT, and EXCH_EVENT.
EXCH_BID_DEPTH_SNAPSHOT_EVENT	Represents a combination of DEPTH_SNAPSHOT_EVENT, BUY_EVENT, and EXCH_EVENT.
EXCH_BUY_TRADE_EVENT	Represents a combination of EXCH_TRADE_EVENT and BUY_EVENT.
EXCH_CANCEL_ORDER_EVENT	Represents a combination of EXCH_EVENT and CANCEL_ORDER_EVENT.
EXCH_DEPTH_CLEAR_EVENT	Represents a combination of DEPTH_CLEAR_EVENT, and EXCH_EVENT.
EXCH_EVENT	Indicates that it is a valid event to be handled by the exchange processor at the exchange timestamp.
EXCH_FILL_EVENT	Represents a combination of EXCH_EVENT and FILL_EVENT.
EXCH_MODIFY_ORDER_EVENT	Represents a combination of EXCH_EVENT and MODIFY_ORDER_EVENT.
EXCH_SELL_TRADE_EVENT	Represents a combination of EXCH_TRADE_EVENT and SELL_EVENT.
EXCH_TRADE_EVENT	Represents a combination of TRADE_EVENT, and EXCH_EVENT.
FILL_EVENT	Indicates that an order in the order book has been filled.
LOCAL_ADD_ORDER_EVENT	Represents a combination of LOCAL_EVENT and ADD_ORDER_EVENT.
LOCAL_ASK_ADD_ORDER_EVENT	Represents a combination of SELL_EVENT and LOCAL_ADD_ORDER_EVENT.
LOCAL_ASK_DEPTH_BBO_EVENT	Represents a combination of DEPTH_BBO_EVENT, SELL_EVENT, and LOCAL_EVENT.
LOCAL_ASK_DEPTH_CLEAR_EVENT	Represents a combination of DEPTH_CLEAR_EVENT, SELL_EVENT, and LOCAL_EVENT.
LOCAL_ASK_DEPTH_EVENT	Represents a combination of DEPTH_EVENT, SELL_EVENT, and LOCAL_EVENT.
LOCAL_ASK_DEPTH_SNAPSHOT_EVENT	Represents a combination of DEPTH_SNAPSHOT_EVENT, SELL_EVENT, and LOCAL_EVENT.
LOCAL_BID_ADD_ORDER_EVENT	Represents a combination of BUY_EVENT and LOCAL_ADD_ORDER_EVENT.
LOCAL_BID_DEPTH_BBO_EVENT	Represents a combination of DEPTH_BBO_EVENT, BUY_EVENT, and LOCAL_EVENT.
LOCAL_BID_DEPTH_CLEAR_EVENT	Represents a combination of DEPTH_CLEAR_EVENT, BUY_EVENT, and LOCAL_EVENT.
LOCAL_BID_DEPTH_EVENT	Represents a combination of a DEPTH_EVENT, BUY_EVENT, and LOCAL_EVENT.
LOCAL_BID_DEPTH_SNAPSHOT_EVENT	Represents a combination of DEPTH_SNAPSHOT_EVENT, BUY_EVENT, and LOCAL_EVENT.
LOCAL_BUY_TRADE_EVENT	Represents a combination of LOCAL_TRADE_EVENT and BUY_EVENT.
LOCAL_CANCEL_ORDER_EVENT	Represents a combination of LOCAL_EVENT and CANCEL_ORDER_EVENT.
LOCAL_DEPTH_CLEAR_EVENT	Represents a combination of DEPTH_CLEAR_EVENT, and LOCAL_EVENT.
LOCAL_EVENT	Indicates that it is a valid event to be handled by the local processor at the local timestamp.
LOCAL_FILL_EVENT	Represents a combination of LOCAL_EVENT and FILL_EVENT.
LOCAL_MODIFY_ORDER_EVENT	Represents a combination of LOCAL_EVENT and MODIFY_ORDER_EVENT.
LOCAL_SELL_TRADE_EVENT	Represents a combination of LOCAL_TRADE_EVENT and SELL_EVENT.
LOCAL_TRADE_EVENT	Represents a combination of TRADE_EVENT, and LOCAL_EVENT.
MODIFY_ORDER_EVENT	Indicates that an order in the order book has been modified.
SELL_EVENT	Indicates a sell, with specific meaning that can vary depending on the situation. For example, when combined with a depth event, it means an ask-side event, while when combined with a trade event, it means that the trade initiator is a seller.
TRADE_EVENT	Indicates that a trade occurs in the market.
UNTIL_END_OF_DATA	Indicates that one should continue until the end of the data.
